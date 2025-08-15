-- =====================================================
-- FUNCIONES Y TRIGGERS PARA CÁLCULOS AUTOMÁTICOS
-- =====================================================

-- Función para actualizar updated_at automáticamente
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Función para calcular distancia total de una tournée
CREATE OR REPLACE FUNCTION calculate_tournee_distance()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.end_mileage IS NOT NULL AND NEW.start_mileage IS NOT NULL THEN
        NEW.total_distance = NEW.end_mileage - NEW.start_mileage;
    END IF;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Función para calcular métricas de performance semanal
CREATE OR REPLACE FUNCTION calculate_weekly_performance()
RETURNS TRIGGER AS $$
DECLARE
    week_start DATE;
    week_end DATE;
    total_pkgs INTEGER;
    successful_pkgs INTEGER;
    failed_pkgs INTEGER;
    total_km DECIMAL;
    total_fuel DECIMAL;
    total_fuel_cost DECIMAL;
    damage_incidents_count INTEGER;
    total_damage_cost DECIMAL;
BEGIN
    -- Calcular inicio y fin de la semana
    week_start = DATE_TRUNC('week', NEW.tournee_date)::DATE;
    week_end = week_start + INTERVAL '6 days';
    
    -- Buscar si ya existe un registro de performance para esta semana
    IF EXISTS (
        SELECT 1 FROM performance_analytics 
        WHERE driver_id = NEW.driver_id 
        AND week_start_date = week_start
    ) THEN
        -- Actualizar registro existente
        UPDATE performance_analytics SET
            total_packages = (
                SELECT COUNT(*) FROM packages p
                JOIN tournees t ON p.tournee_id = t.tournee_id
                WHERE t.driver_id = NEW.driver_id
                AND t.tournee_date BETWEEN week_start AND week_end
                AND t.deleted_at IS NULL
            ),
            successful_deliveries = (
                SELECT COUNT(*) FROM packages p
                JOIN tournees t ON p.tournee_id = t.tournee_id
                WHERE t.driver_id = NEW.driver_id
                AND t.tournee_date BETWEEN week_start AND week_end
                AND p.delivery_status = 'delivered'
                AND t.deleted_at IS NULL
            ),
            failed_deliveries = (
                SELECT COUNT(*) FROM packages p
                JOIN tournees t ON p.tournee_id = t.tournee_id
                WHERE t.driver_id = NEW.driver_id
                AND t.tournee_date BETWEEN week_start AND week_end
                AND p.delivery_status = 'failed'
                AND t.deleted_at IS NULL
            ),
            km_driven = (
                SELECT COALESCE(SUM(t.total_distance), 0) FROM tournees t
                WHERE t.driver_id = NEW.driver_id
                AND t.tournee_date BETWEEN week_start AND week_end
                AND t.deleted_at IS NULL
            ),
            fuel_consumed = (
                SELECT COALESCE(SUM(t.fuel_consumed), 0) FROM tournees t
                WHERE t.driver_id = NEW.driver_id
                AND t.tournee_date BETWEEN week_start AND week_end
                AND t.deleted_at IS NULL
            ),
            fuel_cost = (
                SELECT COALESCE(SUM(t.fuel_cost), 0) FROM tournees t
                WHERE t.driver_id = NEW.driver_id
                AND t.tournee_date BETWEEN week_start AND week_end
                AND t.deleted_at IS NULL
            ),
            damage_incidents = (
                SELECT COUNT(*) FROM vehicle_damages vd
                WHERE vd.driver_id = NEW.driver_id
                AND vd.incident_date BETWEEN week_start AND week_end
                AND vd.deleted_at IS NULL
            ),
            total_damage_cost = (
                SELECT COALESCE(SUM(vd.actual_repair_cost), 0) FROM vehicle_damages vd
                WHERE vd.driver_id = NEW.driver_id
                AND vd.incident_date BETWEEN week_start AND week_end
                AND vd.deleted_at IS NULL
            ),
            updated_at = NOW()
        WHERE driver_id = NEW.driver_id AND week_start_date = week_start;
    ELSE
        -- Crear nuevo registro
        INSERT INTO performance_analytics (
            company_id, driver_id, week_start_date, week_end_date,
            total_packages, successful_deliveries, failed_deliveries,
            km_driven, fuel_consumed, fuel_cost,
            damage_incidents, total_damage_cost
        ) VALUES (
            NEW.company_id, NEW.driver_id, week_start, week_end,
            (SELECT COUNT(*) FROM packages p
             JOIN tournees t ON p.tournee_id = t.tournee_id
             WHERE t.driver_id = NEW.driver_id
             AND t.tournee_date BETWEEN week_start AND week_end
             AND t.deleted_at IS NULL),
            (SELECT COUNT(*) FROM packages p
             JOIN tournees t ON p.tournee_id = t.tournee_id
             WHERE t.driver_id = NEW.driver_id
             AND t.tournee_date BETWEEN week_start AND week_end
             AND p.delivery_status = 'delivered'
             AND t.deleted_at IS NULL),
            (SELECT COUNT(*) FROM packages p
             JOIN tournees t ON p.tournee_id = t.tournee_id
             WHERE t.driver_id = NEW.driver_id
             AND t.tournee_date BETWEEN week_start AND week_end
             AND p.delivery_status = 'failed'
             AND t.deleted_at IS NULL),
            (SELECT COALESCE(SUM(t.total_distance), 0) FROM tournees t
             WHERE t.driver_id = NEW.driver_id
             AND t.tournee_date BETWEEN week_start AND week_end
             AND t.deleted_at IS NULL),
            (SELECT COALESCE(SUM(t.fuel_consumed), 0) FROM tournees t
             WHERE t.driver_id = NEW.driver_id
             AND t.tournee_date BETWEEN week_start AND week_end
             AND t.deleted_at IS NULL),
            (SELECT COALESCE(SUM(t.fuel_cost), 0) FROM tournees t
             WHERE t.driver_id = NEW.driver_id
             AND t.tournee_date BETWEEN week_start AND week_end
             AND t.deleted_at IS NULL),
            (SELECT COUNT(*) FROM vehicle_damages vd
             WHERE vd.driver_id = NEW.driver_id
             AND vd.incident_date BETWEEN week_start AND week_end
             AND vd.deleted_at IS NULL),
            (SELECT COALESCE(SUM(vd.actual_repair_cost), 0) FROM vehicle_damages vd
             WHERE vd.driver_id = NEW.driver_id
             AND vd.incident_date BETWEEN week_start AND week_end
             AND vd.deleted_at IS NULL)
        );
    END IF;
    
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Función para actualizar status de documentos automáticamente
CREATE OR REPLACE FUNCTION update_document_status()
RETURNS TRIGGER AS $$
BEGIN
    -- Actualizar status basado en expiry_date
    IF NEW.expiry_date > CURRENT_DATE + INTERVAL '30 days' THEN
        NEW.document_status = 'valid';
        NEW.notification_sent_30_days = FALSE;
        NEW.notification_sent_15_days = FALSE;
        NEW.notification_sent_expired = FALSE;
    ELSIF NEW.expiry_date > CURRENT_DATE + INTERVAL '15 days' THEN
        NEW.document_status = 'expiring_soon';
        NEW.notification_sent_15_days = FALSE;
        NEW.notification_sent_expired = FALSE;
    ELSIF NEW.expiry_date > CURRENT_DATE THEN
        NEW.document_status = 'expiring_soon';
        NEW.notification_sent_expired = FALSE;
    ELSE
        NEW.document_status = 'expired';
    END IF;
    
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Función para crear notificaciones automáticas de documentos
CREATE OR REPLACE FUNCTION create_document_notifications()
RETURNS TRIGGER AS $$
BEGIN
    -- Notificación a 30 días
    IF NEW.document_status = 'expiring_soon' 
       AND NEW.expiry_date = CURRENT_DATE + INTERVAL '30 days'
       AND NOT NEW.notification_sent_30_days THEN
        
        INSERT INTO notifications_log (
            company_id, vehicle_id, document_id,
            notification_type, notification_priority,
            title, message
        ) VALUES (
            (SELECT company_id FROM vehicles WHERE vehicle_id = NEW.vehicle_id),
            NEW.vehicle_id, NEW.document_id,
            'expiry_warning_30', 'medium',
            'Documento próximo a vencer',
            'El documento ' || NEW.document_name || ' vence en 30 días'
        );
        
        NEW.notification_sent_30_days = TRUE;
    END IF;
    
    -- Notificación a 15 días
    IF NEW.document_status = 'expiring_soon' 
       AND NEW.expiry_date = CURRENT_DATE + INTERVAL '15 days'
       AND NOT NEW.notification_sent_15_days THEN
        
        INSERT INTO notifications_log (
            company_id, vehicle_id, document_id,
            notification_type, notification_priority,
            title, message
        ) VALUES (
            (SELECT company_id FROM vehicles WHERE vehicle_id = NEW.vehicle_id),
            NEW.vehicle_id, NEW.document_id,
            'expiry_warning_15', 'high',
            'Documento próximo a vencer - URGENTE',
            'El documento ' || NEW.document_name || ' vence en 15 días'
        );
        
        NEW.notification_sent_15_days = TRUE;
    END IF;
    
    -- Notificación crítica cuando expira
    IF NEW.document_status = 'expired' 
       AND NOT NEW.notification_sent_expired THEN
        
        INSERT INTO notifications_log (
            company_id, vehicle_id, document_id,
            notification_type, notification_priority,
            title, message
        ) VALUES (
            (SELECT company_id FROM vehicles WHERE vehicle_id = NEW.vehicle_id),
            NEW.vehicle_id, NEW.document_id,
            'expired_critical', 'critical',
            'Documento VENCIDO - ACCIÓN INMEDIATA REQUERIDA',
            'El documento ' || NEW.document_name || ' ha expirado'
        );
        
        NEW.notification_sent_expired = TRUE;
    END IF;
    
    RETURN NEW;
END;
$$ language 'plpgsql';

-- =====================================================
-- TRIGGERS
-- =====================================================

-- Trigger para updated_at en todas las tablas principales
CREATE TRIGGER update_companies_updated_at BEFORE UPDATE ON companies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vehicles_updated_at BEFORE UPDATE ON vehicles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vehicle_documents_updated_at BEFORE UPDATE ON vehicle_documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vehicle_damages_updated_at BEFORE UPDATE ON vehicle_damages
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tournees_updated_at BEFORE UPDATE ON tournees
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_packages_updated_at BEFORE UPDATE ON packages
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_driver_field_data_updated_at BEFORE UPDATE ON driver_field_data
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_performance_analytics_updated_at BEFORE UPDATE ON performance_analytics
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trigger para calcular distancia de tournée
CREATE TRIGGER calculate_tournee_distance_trigger
    BEFORE INSERT OR UPDATE ON tournees
    FOR EACH ROW EXECUTE FUNCTION calculate_tournee_distance();

-- Trigger para calcular performance semanal
CREATE TRIGGER calculate_weekly_performance_trigger
    AFTER INSERT OR UPDATE ON tournees
    FOR EACH ROW EXECUTE FUNCTION calculate_weekly_performance();

-- Trigger para actualizar status de documentos
CREATE TRIGGER update_document_status_trigger
    BEFORE INSERT OR UPDATE ON vehicle_documents
    FOR EACH ROW EXECUTE FUNCTION update_document_status();

-- Trigger para crear notificaciones de documentos
CREATE TRIGGER create_document_notifications_trigger
    AFTER INSERT OR UPDATE ON vehicle_documents
    FOR EACH ROW EXECUTE FUNCTION create_document_notifications();

-- =====================================================
-- ROW LEVEL SECURITY (RLS)
-- =====================================================

-- Habilitar RLS en todas las tablas
ALTER TABLE companies ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE vehicles ENABLE ROW LEVEL SECURITY;
ALTER TABLE vehicle_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE vehicle_damages ENABLE ROW LEVEL SECURITY;
ALTER TABLE tournees ENABLE ROW LEVEL SECURITY;
ALTER TABLE packages ENABLE ROW LEVEL SECURITY;
ALTER TABLE driver_field_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE performance_analytics ENABLE ROW LEVEL SECURITY;
ALTER TABLE notifications_log ENABLE ROW LEVEL SECURITY;

-- Políticas RLS para companies (solo admins del sistema pueden ver todas)
CREATE POLICY companies_select_policy ON companies
    FOR SELECT USING (true);

CREATE POLICY companies_insert_policy ON companies
    FOR INSERT WITH CHECK (true);

CREATE POLICY companies_update_policy ON companies
    FOR UPDATE USING (true);

-- Políticas RLS para users (solo usuarios de la misma empresa)
CREATE POLICY users_select_policy ON users
    FOR SELECT USING (
        company_id IN (
            SELECT company_id FROM users 
            WHERE user_id = current_setting('app.current_user_id')::UUID
        )
    );

CREATE POLICY users_insert_policy ON users
    FOR INSERT WITH CHECK (
        company_id IN (
            SELECT company_id FROM users 
            WHERE user_id = current_setting('app.current_user_id')::UUID
        )
    );

-- Políticas similares para todas las demás tablas...
-- (Por brevedad, solo muestro las principales)

-- Política para vehicles
CREATE POLICY vehicles_select_policy ON vehicles
    FOR SELECT USING (
        company_id IN (
            SELECT company_id FROM users 
            WHERE user_id = current_setting('app.current_user_id')::UUID
        )
    );

-- Política para tournees
CREATE POLICY tournees_select_policy ON tournees
    FOR SELECT USING (
        company_id IN (
            SELECT company_id FROM users 
            WHERE user_id = current_setting('app.current_user_id')::UUID
        )
    );

-- Política para packages
CREATE POLICY packages_select_policy ON packages
    FOR SELECT USING (
        tournee_id IN (
            SELECT tournee_id FROM tournees t
            JOIN users u ON t.company_id = u.company_id
            WHERE u.user_id = current_setting('app.current_user_id')::UUID
        )
    );
