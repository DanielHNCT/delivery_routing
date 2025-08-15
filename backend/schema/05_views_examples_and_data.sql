-- =====================================================
-- VIEWS ÚTILES PARA DASHBOARDS Y REPORTES
-- =====================================================

-- View para dashboard de empresa
CREATE OR REPLACE VIEW company_dashboard AS
SELECT 
    c.company_id,
    c.name as company_name,
    c.subscription_plan,
    c.subscription_status,
    COUNT(DISTINCT u.user_id) as total_users,
    COUNT(DISTINCT CASE WHEN u.user_type = 'admin' THEN u.user_id END) as admin_count,
    COUNT(DISTINCT CASE WHEN u.user_type = 'driver' THEN u.user_id END) as driver_count,
    COUNT(DISTINCT v.vehicle_id) as total_vehicles,
    COUNT(DISTINCT CASE WHEN v.vehicle_status = 'active' THEN v.vehicle_id END) as active_vehicles,
    COUNT(DISTINCT t.tournee_id) as total_tournees,
    COUNT(DISTINCT CASE WHEN t.tournee_status = 'completed' THEN t.tournee_id END) as completed_tournees,
    COALESCE(SUM(t.total_distance), 0) as total_km_driven,
    COALESCE(SUM(t.fuel_consumed), 0) as total_fuel_consumed,
    COALESCE(SUM(t.fuel_cost), 0) as total_fuel_cost
FROM companies c
LEFT JOIN users u ON c.company_id = u.company_id AND u.deleted_at IS NULL
LEFT JOIN vehicles v ON c.company_id = v.company_id AND v.deleted_at IS NULL
LEFT JOIN tournees t ON c.company_id = t.company_id AND t.deleted_at IS NULL
GROUP BY c.company_id, c.name, c.subscription_plan, c.subscription_status;

-- View para performance de choferes
CREATE OR REPLACE VIEW driver_performance_summary AS
SELECT 
    u.user_id,
    u.full_name as driver_name,
    c.name as company_name,
    u.tournee_number,
    COUNT(DISTINCT t.tournee_id) as total_tournees,
    COUNT(DISTINCT CASE WHEN t.tournee_status = 'completed' THEN t.tournee_id END) as completed_tournees,
    COALESCE(SUM(t.total_distance), 0) as total_km_driven,
    COALESCE(SUM(t.fuel_consumed), 0) as total_fuel_consumed,
    COALESCE(SUM(t.fuel_cost), 0) as total_fuel_cost,
    COALESCE(AVG(t.route_optimization_score), 0) as avg_optimization_score,
    COUNT(DISTINCT vd.damage_id) as total_damages,
    COALESCE(SUM(vd.actual_repair_cost), 0) as total_damage_cost,
    COUNT(DISTINCT p.package_id) as total_packages,
    COUNT(DISTINCT CASE WHEN p.delivery_status = 'delivered' THEN p.package_id END) as successful_deliveries,
    ROUND(
        (COUNT(DISTINCT CASE WHEN p.delivery_status = 'delivered' THEN p.package_id END)::DECIMAL / 
         NULLIF(COUNT(DISTINCT p.package_id), 0)) * 100, 2
    ) as delivery_success_rate
FROM users u
JOIN companies c ON u.company_id = c.company_id
LEFT JOIN tournees t ON u.user_id = t.driver_id AND t.deleted_at IS NULL
LEFT JOIN vehicle_damages vd ON u.user_id = vd.driver_id AND vd.deleted_at IS NULL
LEFT JOIN packages p ON t.tournee_id = p.tournee_id AND p.deleted_at IS NULL
WHERE u.user_type = 'driver' AND u.deleted_at IS NULL
GROUP BY u.user_id, u.full_name, c.name, u.tournee_number;

-- View para documentos próximos a vencer
CREATE OR REPLACE VIEW expiring_documents AS
SELECT 
    vd.document_id,
    vd.document_name,
    vd.document_type,
    vd.expiry_date,
    vd.document_status,
    v.license_plate,
    v.brand,
    v.model,
    c.name as company_name,
    CASE 
        WHEN vd.expiry_date <= CURRENT_DATE THEN 'EXPIRED'
        WHEN vd.expiry_date <= CURRENT_DATE + INTERVAL '7 days' THEN 'CRITICAL'
        WHEN vd.expiry_date <= CURRENT_DATE + INTERVAL '15 days' THEN 'HIGH'
        WHEN vd.expiry_date <= CURRENT_DATE + INTERVAL '30 days' THEN 'MEDIUM'
        ELSE 'LOW'
    END as priority_level,
    CURRENT_DATE - vd.expiry_date as days_overdue
FROM vehicle_documents vd
JOIN vehicles v ON vd.vehicle_id = v.vehicle_id
JOIN companies c ON v.company_id = c.company_id
WHERE vd.expiry_date <= CURRENT_DATE + INTERVAL '30 days'
AND vd.deleted_at IS NULL
ORDER BY vd.expiry_date ASC;

-- View para analytics semanales consolidados
CREATE OR REPLACE VIEW weekly_analytics_summary AS
SELECT 
    c.company_id,
    c.name as company_name,
    pa.week_start_date,
    pa.week_end_date,
    COUNT(DISTINCT pa.driver_id) as active_drivers,
    SUM(pa.total_packages) as total_packages,
    SUM(pa.successful_deliveries) as total_successful_deliveries,
    SUM(pa.failed_deliveries) as total_failed_deliveries,
    ROUND(
        (SUM(pa.successful_deliveries)::DECIMAL / 
         NULLIF(SUM(pa.total_packages), 0)) * 100, 2
    ) as overall_success_rate,
    SUM(pa.km_driven) as total_km_driven,
    SUM(pa.fuel_consumed) as total_fuel_consumed,
    SUM(pa.fuel_cost) as total_fuel_cost,
    ROUND(
        SUM(pa.km_driven) / NULLIF(SUM(pa.fuel_consumed), 0), 2
    ) as overall_fuel_efficiency,
    SUM(pa.damage_incidents) as total_damage_incidents,
    SUM(pa.total_damage_cost) as total_damage_cost
FROM performance_analytics pa
JOIN companies c ON pa.company_id = c.company_id
WHERE c.deleted_at IS NULL
GROUP BY c.company_id, c.name, pa.week_start_date, pa.week_end_date
ORDER BY c.company_id, pa.week_start_date DESC;

-- =====================================================
-- QUERIES DE EJEMPLO PARA CASOS DE USO COMUNES
-- =====================================================

-- 1. Obtener tournée del día para un chofer específico
-- Ejemplo de uso: App móvil del chofer
/*
SELECT 
    t.tournee_id,
    t.tournee_date,
    t.tournee_number,
    t.start_location,
    t.end_location,
    t.start_mileage,
    t.end_mileage,
    t.total_distance,
    t.fuel_consumed,
    t.fuel_cost,
    t.route_optimization_score,
    v.license_plate,
    v.brand,
    v.model,
    COUNT(p.package_id) as total_packages,
    COUNT(CASE WHEN p.delivery_status = 'delivered' THEN 1 END) as delivered_packages,
    COUNT(CASE WHEN p.delivery_status = 'pending' THEN 1 END) as pending_packages
FROM tournees t
JOIN vehicles v ON t.vehicle_id = v.vehicle_id
LEFT JOIN packages p ON t.tournee_id = p.tournee_id AND p.deleted_at IS NULL
WHERE t.driver_id = $1 
AND t.tournee_date = CURRENT_DATE
AND t.deleted_at IS NULL
GROUP BY t.tournee_id, t.tournee_date, t.tournee_number, t.start_location, 
         t.end_location, t.start_mileage, t.end_mileage, t.total_distance,
         t.fuel_consumed, t.fuel_cost, t.route_optimization_score,
         v.license_plate, v.brand, v.model;
*/

-- 2. Obtener paquetes de una tournée con información de entrega
-- Ejemplo de uso: Lista de paquetes para el chofer
/*
SELECT 
    p.package_id,
    p.tracking_number,
    p.external_tracking_number,
    p.delivery_status,
    p.delivery_address,
    p.recipient_name,
    p.recipient_phone,
    p.delivery_instructions,
    p.delivery_attempts,
    p.failure_reason,
    p.failure_notes,
    p.driver_notes,
    dfd.door_codes,
    dfd.access_instructions,
    dfd.security_notes,
    dfd.mailbox_location,
    dfd.mailbox_working
FROM packages p
LEFT JOIN driver_field_data dfd ON p.delivery_address = dfd.address 
    AND dfd.company_id = (SELECT company_id FROM tournees WHERE tournee_id = $1)
WHERE p.tournee_id = $1
AND p.deleted_at IS NULL
ORDER BY p.delivery_status, p.delivery_address;
*/

-- 3. Obtener performance semanal de un chofer
-- Ejemplo de uso: Dashboard de rendimiento
/*
SELECT 
    pa.week_start_date,
    pa.week_end_date,
    pa.total_packages,
    pa.successful_deliveries,
    pa.failed_deliveries,
    pa.delivery_success_rate,
    pa.km_driven,
    pa.fuel_consumed,
    pa.fuel_cost,
    pa.fuel_efficiency,
    pa.damage_incidents,
    pa.total_damage_cost,
    pa.damage_score,
    pa.efficiency_ratio,
    pa.cost_per_package,
    pa.anomaly_flags
FROM performance_analytics pa
WHERE pa.driver_id = $1
AND pa.week_start_date >= CURRENT_DATE - INTERVAL '12 weeks'
ORDER BY pa.week_start_date DESC;
*/

-- 4. Obtener alertas y notificaciones no leídas
-- Ejemplo de uso: Sistema de notificaciones
/*
SELECT 
    nl.notification_id,
    nl.notification_type,
    nl.notification_priority,
    nl.title,
    nl.message,
    nl.sent_date,
    nl.read_status,
    v.license_plate,
    vd.document_name,
    u.full_name as driver_name
FROM notifications_log nl
LEFT JOIN vehicles v ON nl.vehicle_id = v.vehicle_id
LEFT JOIN vehicle_documents vd ON nl.document_id = vd.document_id
LEFT JOIN users u ON nl.driver_id = u.user_id
WHERE nl.company_id = $1
AND nl.read_status = FALSE
ORDER BY 
    CASE nl.notification_priority
        WHEN 'critical' THEN 1
        WHEN 'high' THEN 2
        WHEN 'medium' THEN 3
        WHEN 'low' THEN 4
    END,
    nl.sent_date DESC;
*/

-- 5. Obtener resumen de daños por vehículo
-- Ejemplo de uso: Reporte de mantenimiento
/*
SELECT 
    v.vehicle_id,
    v.license_plate,
    v.brand,
    v.model,
    v.current_mileage,
    v.total_damage_cost,
    v.damage_incidents_count,
    COUNT(vd.damage_id) as recent_damages,
    COALESCE(SUM(vd.actual_repair_cost), 0) as recent_repair_cost,
    MAX(vd.incident_date) as last_incident_date,
    vd.damage_type,
    vd.damage_status
FROM vehicles v
LEFT JOIN vehicle_damages vd ON v.vehicle_id = vd.vehicle_id 
    AND vd.incident_date >= CURRENT_DATE - INTERVAL '6 months'
    AND vd.deleted_at IS NULL
WHERE v.company_id = $1
AND v.deleted_at IS NULL
GROUP BY v.vehicle_id, v.license_plate, v.brand, v.model, 
         v.current_mileage, v.total_damage_cost, v.damage_incidents_count,
         vd.damage_type, vd.damage_status
ORDER BY v.total_damage_cost DESC;
*/

-- =====================================================
-- DATOS DE PRUEBA PARA DESARROLLO
-- =====================================================

-- Insertar empresa de prueba
INSERT INTO companies (name, address, subscription_plan, max_drivers, max_vehicles) VALUES
('Delivery Express SAS', '123 Rue de la Logistique, 75001 Paris', 'premium', 15, 8);

-- Insertar usuarios de prueba
INSERT INTO users (company_id, user_type, username, password_hash, full_name, email, tournee_number) VALUES
-- Admin
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'admin', 'admin1', 'hash_admin1', 'Jean Dupont', 'jean@delivery-express.fr', NULL),
-- Drivers
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'driver', 'driver1', 'hash_driver1', 'Pierre Martin', 'pierre@delivery-express.fr', 'D001'),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'driver', 'driver2', 'hash_driver2', 'Marie Dubois', 'marie@delivery-express.fr', 'D002');

-- Insertar vehículos de prueba
INSERT INTO vehicles (company_id, license_plate, brand, model, year, current_mileage, fuel_type, weekly_fuel_allocation) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'AB-123-CD', 'Renault', 'Master', 2020, 45000, 'diesel', 40.0),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'EF-456-GH', 'Peugeot', 'Boxer', 2021, 32000, 'diesel', 35.0);

-- Insertar documentos de vehículos de prueba
INSERT INTO vehicle_documents (vehicle_id, document_type, document_name, expiry_date, document_status) VALUES
((SELECT vehicle_id FROM vehicles WHERE license_plate = 'AB-123-CD'), 'technical_control', 'Contrôle Technique', '2024-12-31', 'valid'),
((SELECT vehicle_id FROM vehicles WHERE license_plate = 'AB-123-CD'), 'insurance', 'Assurance RC', '2024-06-30', 'expiring_soon'),
((SELECT vehicle_id FROM vehicles WHERE license_plate = 'EF-456-GH'), 'technical_control', 'Contrôle Technique', '2024-11-15', 'valid');

-- Insertar tournée de prueba
INSERT INTO tournees (company_id, driver_id, vehicle_id, tournee_date, tournee_number, start_location, end_location, tournee_status, start_mileage, end_mileage, total_distance, fuel_consumed, fuel_cost) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 
 (SELECT user_id FROM users WHERE username = 'driver1'), 
 (SELECT vehicle_id FROM vehicles WHERE license_plate = 'AB-123-CD'),
 '2024-01-15', 'T001', 'Entrepôt Paris Nord', 'Entrepôt Paris Nord', 'completed', 45000, 45250, 250, 25.5, 45.90);

-- Insertar paquetes de prueba
INSERT INTO packages (tournee_id, tracking_number, external_tracking_number, delivery_status, delivery_address, recipient_name, delivery_attempts) VALUES
((SELECT tournee_id FROM tournees WHERE tournee_number = 'T001'), 'PKG001', 'CP123456789', 'delivered', '15 Rue de la Paix, 75001 Paris', 'Sophie Bernard', 1),
((SELECT tournee_id FROM tournees WHERE tournee_number = 'T001'), 'PKG002', 'CP987654321', 'delivered', '28 Avenue des Champs, 75008 Paris', 'Michel Durand', 1),
((SELECT tournee_id FROM tournees WHERE tournee_number = 'T001'), 'PKG003', 'CP456789123', 'failed', '7 Boulevard Saint-Germain, 75006 Paris', 'Anne Moreau', 2);

-- Insertar datos de campo de prueba
INSERT INTO driver_field_data (company_id, driver_id, address, postal_code, city, door_codes, mailbox_location, mailbox_working, confidence_score) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
 (SELECT user_id FROM users WHERE username = 'driver1'),
 '15 Rue de la Paix, 75001 Paris', '75001', 'Paris', 'Code: 1234', 'Buzón principal', true, 5),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
 (SELECT user_id FROM users WHERE username = 'driver1'),
 '28 Avenue des Champs, 75008 Paris', '75008', 'Paris', 'Intercom: DURAND', 'Buzón individual', true, 4);

-- =====================================================
-- FUNCIONES UTILITARIAS ADICIONALES
-- =====================================================

-- Función para obtener estadísticas de una empresa
CREATE OR REPLACE FUNCTION get_company_stats(company_uuid UUID)
RETURNS TABLE(
    total_drivers BIGINT,
    total_vehicles BIGINT,
    total_tournees BIGINT,
    total_packages BIGINT,
    total_km_driven DECIMAL,
    total_fuel_cost DECIMAL,
    avg_delivery_success_rate DECIMAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(DISTINCT u.user_id)::BIGINT,
        COUNT(DISTINCT v.vehicle_id)::BIGINT,
        COUNT(DISTINCT t.tournee_id)::BIGINT,
        COUNT(DISTINCT p.package_id)::BIGINT,
        COALESCE(SUM(t.total_distance), 0),
        COALESCE(SUM(t.fuel_cost), 0),
        ROUND(
            (COUNT(CASE WHEN p.delivery_status = 'delivered' THEN 1 END)::DECIMAL / 
             NULLIF(COUNT(p.package_id), 0)) * 100, 2
        )
    FROM companies c
    LEFT JOIN users u ON c.company_id = u.company_id AND u.user_type = 'driver' AND u.deleted_at IS NULL
    LEFT JOIN vehicles v ON c.company_id = v.company_id AND v.deleted_at IS NULL
    LEFT JOIN tournees t ON c.company_id = t.company_id AND t.deleted_at IS NULL
    LEFT JOIN packages p ON t.tournee_id = p.tournee_id AND p.deleted_at IS NULL
    WHERE c.company_id = company_uuid
    AND c.deleted_at IS NULL;
END;
$$ LANGUAGE plpgsql;

-- Función para limpiar datos antiguos (mantenimiento)
CREATE OR REPLACE FUNCTION cleanup_old_data(months_to_keep INTEGER DEFAULT 24)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER := 0;
BEGIN
    -- Limpiar tournees antiguas (soft delete)
    UPDATE tournees 
    SET deleted_at = NOW() 
    WHERE tournee_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND deleted_at IS NULL;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    -- Limpiar packages antiguos
    UPDATE packages 
    SET deleted_at = NOW() 
    WHERE delivery_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND deleted_at IS NULL;
    
    -- Limpiar analytics antiguos
    DELETE FROM performance_analytics 
    WHERE week_start_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep;
    
    -- Limpiar notificaciones antiguas leídas
    DELETE FROM notifications_log 
    WHERE sent_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND read_status = TRUE;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;
