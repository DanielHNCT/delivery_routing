-- =====================================================
-- NIVEL 4A: PACKAGES (Paquetes del día)
-- =====================================================

CREATE TYPE delivery_status AS ENUM (
    'pending', 
    'in_transit', 
    'out_for_delivery', 
    'delivered', 
    'failed',
    'returned',
    'cancelled'
);

CREATE TYPE delivery_failure_reason AS ENUM (
    'recipient_not_home',
    'wrong_address',
    'package_damaged',
    'refused_delivery',
    'security_restriction',
    'weather_conditions',
    'vehicle_breakdown',
    'driver_emergency'
);

CREATE TABLE packages (
    package_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tournee_id UUID NOT NULL REFERENCES tournees(tournee_id) ON DELETE CASCADE,
    
    -- Información del paquete
    tracking_number VARCHAR(100) NOT NULL,
    external_tracking_number VARCHAR(100), -- Para APIs externas como Colis Privé
    package_type VARCHAR(100),
    package_weight DECIMAL(6,2),
    package_dimensions VARCHAR(50), -- formato: "LxWxH cm"
    
    -- Estado de entrega
    delivery_status delivery_status NOT NULL DEFAULT 'pending',
    delivery_date DATE,
    delivery_time TIME,
    delivery_attempts INTEGER DEFAULT 0,
    
    -- Información de entrega
    recipient_name VARCHAR(255),
    recipient_phone VARCHAR(20),
    delivery_address TEXT NOT NULL,
    delivery_instructions TEXT,
    
    -- Fallos y reintentos
    failure_reason delivery_failure_reason,
    failure_notes TEXT,
    reschedule_date DATE,
    
    -- Evidencia de entrega
    delivery_photo TEXT,
    signature_required BOOLEAN DEFAULT FALSE,
    signature_image TEXT,
    
    -- Notas del chofer
    driver_notes TEXT,
    package_condition TEXT,
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT unique_tracking_per_tournee UNIQUE (tournee_id, tracking_number),
    CONSTRAINT valid_delivery_attempts CHECK (delivery_attempts >= 0),
    CONSTRAINT valid_package_weight CHECK (package_weight > 0 OR package_weight IS NULL)
);

-- Índices para packages
CREATE INDEX idx_packages_tournee_id ON packages(tournee_id);
CREATE INDEX idx_packages_tracking_number ON packages(tracking_number);
CREATE INDEX idx_packages_external_tracking ON packages(external_tracking_number);
CREATE INDEX idx_packages_status ON packages(delivery_status);
CREATE INDEX idx_packages_delivery_date ON packages(delivery_date);
CREATE INDEX idx_packages_deleted_at ON packages(deleted_at);
CREATE INDEX idx_packages_status_date ON packages(delivery_status, delivery_date);

-- =====================================================
-- NIVEL 4B: DRIVER_FIELD_DATA (Datos crowdsourced por choferes)
-- =====================================================

CREATE TABLE driver_field_data (
    data_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    driver_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    
    -- Ubicación
    address TEXT NOT NULL,
    postal_code VARCHAR(20),
    city VARCHAR(100),
    coordinates POINT, -- PostGIS point para geolocalización
    
    -- Información de acceso
    door_codes TEXT, -- Códigos de puerta, buzones, etc.
    access_instructions TEXT,
    security_notes TEXT,
    
    -- Estado del buzón
    mailbox_location TEXT,
    mailbox_working BOOLEAN,
    mailbox_issues TEXT,
    
    -- Horarios y preferencias
    preferred_delivery_time VARCHAR(100),
    delivery_restrictions TEXT,
    special_instructions TEXT,
    
    -- Calidad de los datos
    confidence_score INTEGER CHECK (confidence_score >= 1 AND confidence_score <= 5),
    data_source VARCHAR(50) DEFAULT 'driver_input',
    verification_count INTEGER DEFAULT 1,
    
    -- Última actualización
    last_updated_by UUID REFERENCES users(user_id) ON DELETE SET NULL,
    last_verified_date DATE,
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT unique_address_per_company UNIQUE (company_id, address),
    CONSTRAINT valid_confidence_score CHECK (confidence_score >= 1 AND confidence_score <= 5)
);

-- Índices para driver_field_data
CREATE INDEX idx_driver_field_data_company_id ON driver_field_data(company_id);
CREATE INDEX idx_driver_field_data_driver_id ON driver_field_data(driver_id);
CREATE INDEX idx_driver_field_data_address ON driver_field_data(address);
CREATE INDEX idx_driver_field_data_postal_code ON driver_field_data(postal_code);
CREATE INDEX idx_driver_field_data_city ON driver_field_data(city);
CREATE INDEX idx_driver_field_data_coordinates ON driver_field_data USING GIST(coordinates);
CREATE INDEX idx_driver_field_data_deleted_at ON driver_field_data(deleted_at);

-- =====================================================
-- NIVEL 5A: PERFORMANCE_ANALYTICS (Calculado automáticamente)
-- =====================================================

CREATE TABLE performance_analytics (
    analytics_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    driver_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    
    -- Período de análisis
    week_start_date DATE NOT NULL,
    week_end_date DATE NOT NULL,
    
    -- Métricas de entrega
    total_packages INTEGER NOT NULL DEFAULT 0,
    successful_deliveries INTEGER NOT NULL DEFAULT 0,
    failed_deliveries INTEGER NOT NULL DEFAULT 0,
    delivery_success_rate DECIMAL(5,2), -- Calculado automáticamente
    
    -- Métricas de distancia y combustible
    km_driven DECIMAL(8,2) NOT NULL DEFAULT 0,
    fuel_consumed DECIMAL(5,2) NOT NULL DEFAULT 0,
    fuel_cost DECIMAL(8,2) NOT NULL DEFAULT 0,
    fuel_efficiency DECIMAL(6,2), -- km/litro, calculado automáticamente
    
    -- Métricas de tiempo
    total_working_hours DECIMAL(4,2),
    average_delivery_time_minutes DECIMAL(5,2),
    route_optimization_score DECIMAL(3,2),
    
    -- Métricas de daños y incidentes
    damage_incidents INTEGER NOT NULL DEFAULT 0,
    total_damage_cost DECIMAL(10,2) NOT NULL DEFAULT 0,
    damage_score DECIMAL(3,2), -- 0.00 a 1.00, calculado automáticamente
    
    -- Métricas de rendimiento
    efficiency_ratio DECIMAL(5,2), -- paquetes/km, calculado automáticamente
    cost_per_package DECIMAL(6,2), -- costo total por paquete
    profit_margin DECIMAL(5,2), -- margen de beneficio
    
    -- Banderas de anomalías
    anomaly_flags JSONB DEFAULT '{}', -- exceso de consumo, baja tasa de entrega, etc.
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT unique_driver_week UNIQUE (driver_id, week_start_date),
    CONSTRAINT valid_week_dates CHECK (week_end_date = week_start_date + INTERVAL '6 days'),
    CONSTRAINT valid_metrics CHECK (
        total_packages >= 0 AND
        successful_deliveries >= 0 AND
        failed_deliveries >= 0 AND
        km_driven >= 0 AND
        fuel_consumed >= 0
    )
);

-- Índices para performance_analytics
CREATE INDEX idx_performance_analytics_company_id ON performance_analytics(company_id);
CREATE INDEX idx_performance_analytics_driver_id ON performance_analytics(driver_id);
CREATE INDEX idx_performance_analytics_week_start ON performance_analytics(week_start_date);
CREATE INDEX idx_performance_analytics_week_end ON performance_analytics(week_end_date);
CREATE INDEX idx_performance_analytics_driver_week ON performance_analytics(driver_id, week_start_date);
CREATE INDEX idx_performance_analytics_company_week ON performance_analytics(company_id, week_start_date);

-- =====================================================
-- NIVEL 5B: NOTIFICATIONS_LOG (Sistema de alertas)
-- =====================================================

CREATE TYPE notification_type AS ENUM (
    'expiry_warning_30', 
    'expiry_warning_15', 
    'expired_critical',
    'damage_incident',
    'performance_alert',
    'fuel_consumption_alert',
    'maintenance_reminder',
    'system_alert'
);

CREATE TYPE notification_priority AS ENUM ('low', 'medium', 'high', 'critical');

CREATE TABLE notifications_log (
    notification_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    admin_id UUID REFERENCES users(user_id) ON DELETE SET NULL,
    document_id UUID REFERENCES vehicle_documents(document_id) ON DELETE SET NULL,
    vehicle_id UUID REFERENCES vehicles(vehicle_id) ON DELETE SET NULL,
    driver_id UUID REFERENCES users(user_id) ON DELETE SET NULL,
    
    -- Información de la notificación
    notification_type notification_type NOT NULL,
    notification_priority notification_priority NOT NULL DEFAULT 'medium',
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    
    -- Estado y seguimiento
    sent_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    read_status BOOLEAN DEFAULT FALSE,
    read_date TIMESTAMP WITH TIME ZONE,
    action_taken TEXT,
    action_date TIMESTAMP WITH TIME ZONE,
    
    -- Metadatos adicionales
    metadata JSONB DEFAULT '{}',
    email_sent BOOLEAN DEFAULT FALSE,
    sms_sent BOOLEAN DEFAULT FALSE,
    push_notification_sent BOOLEAN DEFAULT FALSE,
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT valid_read_dates CHECK (
        (read_date IS NULL) OR 
        (read_date >= sent_date)
    ),
    CONSTRAINT valid_action_dates CHECK (
        (action_date IS NULL) OR 
        (action_date >= sent_date)
    )
);

-- Índices para notifications_log
CREATE INDEX idx_notifications_log_company_id ON notifications_log(company_id);
CREATE INDEX idx_notifications_log_admin_id ON notifications_log(admin_id);
CREATE INDEX idx_notifications_log_type ON notifications_log(notification_type);
CREATE INDEX idx_notifications_log_priority ON notifications_log(notification_priority);
CREATE INDEX idx_notifications_log_sent_date ON notifications_log(sent_date);
CREATE INDEX idx_notifications_log_read_status ON notifications_log(read_status);
CREATE INDEX idx_notifications_log_company_type ON notifications_log(company_id, notification_type);
CREATE INDEX idx_notifications_log_unread ON notifications_log(company_id, read_status) WHERE read_status = FALSE;
