-- =====================================================
-- NIVEL 3A: VEHICLE_DOCUMENTS (Asociados a vehicles)
-- =====================================================

CREATE TYPE document_type AS ENUM (
    'technical_control', 
    'insurance', 
    'carte_grise', 
    'driver_license',
    'vehicle_registration',
    'maintenance_book'
);

CREATE TYPE document_status AS ENUM (
    'valid', 
    'expiring_soon', 
    'expired', 
    'renewal_in_progress',
    'missing'
);

CREATE TABLE vehicle_documents (
    document_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    vehicle_id UUID NOT NULL REFERENCES vehicles(vehicle_id) ON DELETE CASCADE,
    document_type document_type NOT NULL,
    document_number VARCHAR(100),
    document_name VARCHAR(255) NOT NULL,
    
    -- Fechas importantes
    issue_date DATE,
    expiry_date DATE NOT NULL,
    renewal_reminder_date DATE,
    
    -- Estado y notificaciones
    document_status document_status NOT NULL DEFAULT 'valid',
    notification_sent_30_days BOOLEAN DEFAULT FALSE,
    notification_sent_15_days BOOLEAN DEFAULT FALSE,
    notification_sent_expired BOOLEAN DEFAULT FALSE,
    
    -- Archivos y metadatos
    document_path TEXT,
    document_url TEXT,
    file_size BIGINT,
    mime_type VARCHAR(100),
    
    -- Notas y estado
    notes TEXT,
    renewal_notes TEXT,
    insurance_company VARCHAR(100),
    policy_number VARCHAR(100),
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT unique_document_type_per_vehicle UNIQUE (vehicle_id, document_type),
    CONSTRAINT valid_expiry_date CHECK (expiry_date > issue_date OR issue_date IS NULL)
);

-- Índices para vehicle_documents
CREATE INDEX idx_vehicle_documents_vehicle_id ON vehicle_documents(vehicle_id);
CREATE INDEX idx_vehicle_documents_type ON vehicle_documents(document_type);
CREATE INDEX idx_vehicle_documents_status ON vehicle_documents(document_status);
CREATE INDEX idx_vehicle_documents_expiry_date ON vehicle_documents(expiry_date);
CREATE INDEX idx_vehicle_documents_deleted_at ON vehicle_documents(deleted_at);
CREATE INDEX idx_vehicle_documents_expiry_status ON vehicle_documents(expiry_date, document_status);

-- =====================================================
-- NIVEL 3B: VEHICLE_DAMAGES (Daños causados por choferes)
-- =====================================================

CREATE TYPE damage_type AS ENUM (
    'scratch', 
    'dent', 
    'mechanical', 
    'accident', 
    'traffic_fine',
    'vandalism',
    'weather_damage',
    'wear_and_tear'
);

CREATE TYPE damage_status AS ENUM (
    'pending', 
    'assessed', 
    'repaired', 
    'driver_liable',
    'insurance_claim',
    'closed'
);

CREATE TABLE vehicle_damages (
    damage_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    vehicle_id UUID NOT NULL REFERENCES vehicles(vehicle_id) ON DELETE CASCADE,
    driver_id UUID REFERENCES users(user_id) ON DELETE SET NULL,
    tournee_id UUID, -- Puede ser NULL si no está asociado a una tournée específica
    
    -- Información del incidente
    incident_date DATE NOT NULL,
    incident_time TIME,
    damage_type damage_type NOT NULL,
    damage_location VARCHAR(255),
    description TEXT NOT NULL,
    
    -- Costos y responsabilidad
    estimated_repair_cost DECIMAL(10,2),
    actual_repair_cost DECIMAL(10,2),
    responsibility_percentage INTEGER CHECK (responsibility_percentage >= 0 AND responsibility_percentage <= 100),
    driver_deductible DECIMAL(10,2),
    
    -- Estado y seguimiento
    damage_status damage_status NOT NULL DEFAULT 'pending',
    assessment_date DATE,
    repair_date DATE,
    completion_date DATE,
    
    -- Evidencia y documentación
    photo_evidence TEXT[], -- Array de URLs de fotos
    police_report TEXT,
    witness_statements TEXT,
    
    -- Seguro
    insurance_claim BOOLEAN DEFAULT FALSE,
    claim_number VARCHAR(100),
    deductible_amount DECIMAL(10,2),
    insurance_company VARCHAR(100),
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT valid_incident_date CHECK (incident_date <= CURRENT_DATE),
    CONSTRAINT valid_repair_dates CHECK (
        (repair_date IS NULL OR repair_date >= incident_date) AND
        (completion_date IS NULL OR completion_date >= repair_date OR repair_date IS NULL)
    )
);

-- Índices para vehicle_damages
CREATE INDEX idx_vehicle_damages_vehicle_id ON vehicle_damages(vehicle_id);
CREATE INDEX idx_vehicle_damages_driver_id ON vehicle_damages(driver_id);
CREATE INDEX idx_vehicle_damages_tournee_id ON vehicle_damages(tournee_id);
CREATE INDEX idx_vehicle_damages_type ON vehicle_damages(damage_type);
CREATE INDEX idx_vehicle_damages_status ON vehicle_damages(damage_status);
CREATE INDEX idx_vehicle_damages_incident_date ON vehicle_damages(incident_date);
CREATE INDEX idx_vehicle_damages_deleted_at ON vehicle_damages(deleted_at);
CREATE INDEX idx_vehicle_damages_vehicle_driver ON vehicle_damages(vehicle_id, driver_id);

-- =====================================================
-- NIVEL 3C: TOURNEES (Rutas diarias)
-- =====================================================

CREATE TYPE tournee_status AS ENUM (
    'pending', 
    'in_progress', 
    'completed', 
    'cancelled',
    'paused'
);

CREATE TABLE tournees (
    tournee_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    driver_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    vehicle_id UUID NOT NULL REFERENCES vehicles(vehicle_id) ON DELETE CASCADE,
    
    -- Información de la ruta
    tournee_date DATE NOT NULL,
    tournee_number VARCHAR(50),
    start_location TEXT,
    end_location TEXT,
    
    -- Estado operativo
    tournee_status tournee_status NOT NULL DEFAULT 'pending',
    start_time TIMESTAMP WITH TIME ZONE,
    end_time TIMESTAMP WITH TIME ZONE,
    
    -- Métricas de kilometraje y combustible
    start_mileage DECIMAL(10,2),
    end_mileage DECIMAL(10,2),
    total_distance DECIMAL(8,2), -- Calculado automáticamente
    fuel_consumed DECIMAL(5,2),
    fuel_cost DECIMAL(8,2),
    
    -- Inspecciones
    pre_inspection_notes TEXT,
    post_inspection_notes TEXT,
    pre_inspection_photos TEXT[],
    post_inspection_photos TEXT[],
    
    -- Optimización de ruta
    route_optimization_score DECIMAL(3,2), -- 0.00 a 1.00
    estimated_duration_minutes INTEGER,
    actual_duration_minutes INTEGER,
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT unique_tournee_per_driver_date UNIQUE (driver_id, tournee_date),
    CONSTRAINT valid_mileage CHECK (
        (start_mileage IS NULL OR end_mileage IS NULL) OR 
        (end_mileage >= start_mileage)
    ),
    CONSTRAINT valid_times CHECK (
        (start_time IS NULL OR end_time IS NULL) OR 
        (end_time >= start_time)
    )
);

-- Índices para tournees
CREATE INDEX idx_tournees_company_id ON tournees(company_id);
CREATE INDEX idx_tournees_driver_id ON tournees(driver_id);
CREATE INDEX idx_tournees_vehicle_id ON tournees(vehicle_id);
CREATE INDEX idx_tournees_date ON tournees(tournee_date);
CREATE INDEX idx_tournees_status ON tournees(tournee_status);
CREATE INDEX idx_tournees_deleted_at ON tournees(deleted_at);
CREATE INDEX idx_tournees_driver_date ON tournees(driver_id, tournee_date);
CREATE INDEX idx_tournees_company_date ON tournees(company_id, tournee_date);
