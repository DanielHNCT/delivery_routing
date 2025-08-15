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

-- 6. Obtener choferes activos con su ubicación actual
-- Ejemplo de uso: Tracking en tiempo real
/*
SELECT 
    u.user_id,
    u.full_name,
    u.tournee_number,
    u.last_location,
    u.shift_start_time,
    u.shift_end_time,
    u.device_token,
    t.tournee_id,
    t.tournee_status,
    t.route_coordinates,
    t.traffic_conditions,
    t.weather_conditions
FROM users u
LEFT JOIN tournees t ON u.user_id = t.driver_id 
    AND t.tournee_date = CURRENT_DATE
    AND t.deleted_at IS NULL
WHERE u.user_type = 'driver' 
AND u.user_status = 'active'
AND u.company_id = $1
AND u.deleted_at IS NULL
ORDER BY u.shift_start_time;
*/

-- 7. Obtener paquetes entregados con coordenadas y tiempo de entrega
-- Ejemplo de uso: Análisis de eficiencia de rutas
/*
SELECT 
    p.package_id,
    p.tracking_number,
    p.delivery_address,
    p.delivery_coordinates,
    p.delivery_duration_minutes,
    p.delivery_date,
    p.delivery_time,
    t.route_coordinates,
    t.traffic_conditions,
    t.weather_conditions,
    ST_Distance(
        p.delivery_coordinates::geography,
        t.route_coordinates[1]::geography
    ) as distance_from_route_start
FROM packages p
JOIN tournees t ON p.tournee_id = t.tournee_id
WHERE p.delivery_status = 'delivered'
AND p.delivery_coordinates IS NOT NULL
AND t.route_coordinates IS NOT NULL
AND p.deleted_at IS NULL
AND t.deleted_at IS NULL
ORDER BY p.delivery_date DESC, p.delivery_time DESC;
*/

-- 8. Obtener condiciones de tráfico y clima por tournée
-- Ejemplo de uso: Análisis de factores externos
/*
SELECT 
    t.tournee_id,
    t.tournee_date,
    t.tournee_number,
    u.full_name as driver_name,
    t.route_coordinates,
    t.traffic_conditions,
    t.weather_conditions,
    t.actual_duration_minutes,
    t.estimated_duration_minutes,
    CASE 
        WHEN t.actual_duration_minutes > t.estimated_duration_minutes * 1.2 THEN 'DELAYED'
        WHEN t.actual_duration_minutes < t.estimated_duration_minutes * 0.8 THEN 'EARLY'
        ELSE 'ON_TIME'
    END as performance_status
FROM tournees t
JOIN users u ON t.driver_id = u.user_id
WHERE t.company_id = $1
AND t.traffic_conditions IS NOT NULL
AND t.weather_conditions IS NOT NULL
AND t.deleted_at IS NULL
ORDER BY t.tournee_date DESC;
*/

-- =====================================================
-- DATOS DE PRUEBA PARA DESARROLLO
-- =====================================================

-- Insertar empresa de prueba
INSERT INTO companies (name, address, subscription_plan, max_drivers, max_vehicles) VALUES
('Delivery Express SAS', '123 Rue de la Logistique, 75001 Paris', 'premium', 15, 8);

-- Insertar usuarios de prueba
INSERT INTO users (company_id, user_type, username, password_hash, full_name, email, tournee_number, device_token, last_location, shift_start_time, shift_end_time) VALUES
-- Admin
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'admin', 'admin1', 'hash_admin1', 'Jean Dupont', 'jean@delivery-express.fr', NULL, NULL, NULL, NULL, NULL),
-- Drivers
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'driver', 'driver1', 'hash_driver1', 'Pierre Martin', 'pierre@delivery-express.fr', 'D001', 'device_token_pierre_123', point(2.3522, 48.8566), '08:00:00', '17:00:00'),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'driver', 'driver2', 'hash_driver2', 'Marie Dubois', 'marie@delivery-express.fr', 'D002', 'device_token_marie_456', point(2.3522, 48.8566), '09:00:00', '18:00:00');

-- Insertar vehículos de prueba
INSERT INTO vehicles (company_id, license_plate, brand, model, year, current_mileage, fuel_type, weekly_fuel_allocation) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'AB-123-CD', 'Renault', 'Master', 2020, 45000, 'diesel', 40.0),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 'EF-456-GH', 'Peugeot', 'Boxer', 2021, 32000, 'diesel', 35.0);

-- Insertar documentos de vehículos de prueba
INSERT INTO vehicle_documents (company_id, vehicle_id, document_type, document_name, expiry_date, document_status) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), (SELECT vehicle_id FROM vehicles WHERE license_plate = 'AB-123-CD'), 'technical_control', 'Contrôle Technique', '2024-12-31', 'valid'),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), (SELECT vehicle_id FROM vehicles WHERE license_plate = 'AB-123-CD'), 'insurance', 'Assurance RC', '2024-06-30', 'expiring_soon'),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), (SELECT vehicle_id FROM vehicles WHERE license_plate = 'EF-456-GH'), 'technical_control', 'Contrôle Technique', '2024-11-15', 'valid');

-- Insertar tournée de prueba
INSERT INTO tournees (company_id, driver_id, vehicle_id, tournee_date, tournee_number, start_location, end_location, tournee_status, start_mileage, end_mileage, total_distance, fuel_consumed, fuel_cost, route_coordinates, traffic_conditions, weather_conditions) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), 
 (SELECT user_id FROM users WHERE username = 'driver1'), 
 (SELECT vehicle_id FROM vehicles WHERE license_plate = 'AB-123-CD'),
 '2024-01-15', 'T001', 'Entrepôt Paris Nord', 'Entrepôt Paris Nord', 'completed', 45000, 45250, 250, 25.5, 45.90,
 ARRAY['48.8566,2.3522', '48.8606,2.3376', '48.8566,2.3522'],
 '{"congestion_level": "medium", "accidents": 0, "road_works": false}',
 '{"temperature": 15, "weather": "partly_cloudy", "wind_speed": 12, "precipitation": false}');

-- Insertar paquetes de prueba
INSERT INTO packages (company_id, tournee_id, tracking_number, external_tracking_number, delivery_status, delivery_address, recipient_name, delivery_attempts, delivery_coordinates, delivery_duration_minutes, signature_photo) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), (SELECT tournee_id FROM tournees WHERE tournee_number = 'T001'), 'PKG001', 'CP123456789', 'delivered', '15 Rue de la Paix, 75001 Paris', 'Sophie Bernard', 1, point(48.8566, 2.3522), 5, 'signature_photo_pkg001.jpg'),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), (SELECT tournee_id FROM tournees WHERE tournee_number = 'T001'), 'PKG002', 'CP987654321', 'delivered', '28 Avenue des Champs, 75008 Paris', 'Michel Durand', 1, point(48.8700, 2.3077), 7, 'signature_photo_pkg002.jpg'),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'), (SELECT tournee_id FROM tournees WHERE tournee_number = 'T001'), 'PKG003', 'CP456789123', 'failed', '7 Boulevard Saint-Germain, 75006 Paris', 'Anne Moreau', 2, point(48.8534, 2.3488), NULL, NULL);

-- Insertar datos de campo de prueba
INSERT INTO driver_field_data (company_id, driver_id, address, postal_code, city, door_codes, mailbox_location, mailbox_working, confidence_score) VALUES
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
 (SELECT user_id FROM users WHERE username = 'driver1'),
 '15 Rue de la Paix, 75001 Paris', '75001', 'Paris', 'Code: 1234', 'Buzón principal', true, 5),
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
 (SELECT user_id FROM users WHERE username = 'driver1'),
 '28 Avenue des Champs, 75008 Paris', '75008', 'Paris', 'Intercom: DURAND', 'Buzón individual', true, 4);

-- =====================================================
-- DATOS DE PRUEBA PARA INTEGRACIONES CON APIs
-- =====================================================

-- Insertar integración con Colis Privé
INSERT INTO api_integrations (
    company_id, 
    provider_name, 
    provider_display_name, 
    api_credentials, 
    api_endpoint, 
    daily_sync_limit, 
    sync_frequency_hours
) VALUES (
    (SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
    'colis_prive',
    'Colis Privé',
    '{"api_key": "cp_test_key_123", "client_id": "delivery_express", "secret": "test_secret"}',
    'https://api.colisprive.fr/v2',
    2000,
    6
);

-- Insertar integración con Chronopost
INSERT INTO api_integrations (
    company_id, 
    provider_name, 
    provider_display_name, 
    api_credentials, 
    api_endpoint, 
    daily_sync_limit, 
    sync_frequency_hours
) VALUES (
    (SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
    'chronopost',
    'Chronopost',
    '{"api_key": "chrono_test_key_456", "account_number": "12345678", "password": "test_pass"}',
    'https://api.chronopost.fr/ws',
    1500,
    12
);

-- Actualizar tournée de prueba para marcarla como sincronizada desde API
UPDATE tournees SET
    tournee_origin = 'api_sync',
    external_tournee_id = 'CP_T001_20240115',
    integration_id = (SELECT integration_id FROM api_integrations WHERE provider_name = 'colis_prive')
WHERE tournee_number = 'T001';

-- Actualizar paquetes de prueba para marcarlos como sincronizados desde API
UPDATE packages SET
    package_origin = 'api_sync',
    external_package_id = 'CP_PKG001',
    integration_id = (SELECT integration_id FROM api_integrations WHERE provider_name = 'colis_prive')
WHERE tracking_number = 'PKG001';

UPDATE packages SET
    package_origin = 'api_sync',
    external_package_id = 'CP_PKG002',
    integration_id = (SELECT integration_id FROM api_integrations WHERE provider_name = 'colis_prive')
WHERE tracking_number = 'PKG002';

-- Insertar logs de sincronización de prueba
INSERT INTO sync_log (
    company_id,
    integration_id,
    sync_type,
    sync_direction,
    records_processed,
    records_created,
    records_updated,
    sync_status,
    sync_duration_seconds,
    sync_start_time,
    sync_end_time
) VALUES
-- Log para Colis Privé
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
 (SELECT integration_id FROM api_integrations WHERE provider_name = 'colis_prive'),
 'incremental', 'inbound', 15, 3, 12, 'completed', 45,
 NOW() - INTERVAL '2 hours', NOW() - INTERVAL '2 hours' + INTERVAL '45 seconds'),

-- Log para Chronopost
((SELECT company_id FROM companies WHERE name = 'Delivery Express SAS'),
 (SELECT integration_id FROM api_integrations WHERE provider_name = 'chronopost'),
 'full_sync', 'inbound', 8, 2, 6, 'completed', 120,
 NOW() - INTERVAL '6 hours', NOW() - INTERVAL '6 hours' + INTERVAL '2 minutes');

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
    partition_name TEXT;
    partition_date DATE;
    current_month DATE;
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
    
    -- Limpiar analytics antiguos (por particiones)
    DELETE FROM performance_analytics 
    WHERE week_start_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep;
    
    -- Limpiar notificaciones antiguas leídas
    DELETE FROM notifications_log 
    WHERE sent_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND read_status = TRUE;
    
    -- Limpiar datos de campo antiguos
    UPDATE driver_field_data 
    SET deleted_at = NOW() 
    WHERE last_verified_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND deleted_at IS NULL;
    
    -- Limpiar daños de vehículos antiguos (soft delete)
    UPDATE vehicle_damages 
    SET deleted_at = NOW() 
    WHERE incident_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND deleted_at IS NULL;
    
    -- Limpiar documentos de vehículos expirados hace mucho tiempo
    UPDATE vehicle_documents 
    SET deleted_at = NOW() 
    WHERE expiry_date < CURRENT_DATE - INTERVAL '1 month' * (months_to_keep + 6)
    AND deleted_at IS NULL;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Función para limpiar datos muy antiguos (6+ meses) de forma agresiva
CREATE OR REPLACE FUNCTION aggressive_cleanup_old_data(months_to_keep INTEGER DEFAULT 6)
RETURNS TABLE(
    table_name TEXT,
    records_deleted BIGINT,
    action_type TEXT
) AS $$
DECLARE
    deleted_count BIGINT;
BEGIN
    -- Limpiar performance_analytics antiguos (eliminación física)
    DELETE FROM performance_analytics 
    WHERE week_start_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN QUERY SELECT 'performance_analytics'::TEXT, deleted_count, 'DELETE'::TEXT;
    
    -- Limpiar notificaciones muy antiguas (eliminación física)
    DELETE FROM notifications_log 
    WHERE sent_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN QUERY SELECT 'notifications_log'::TEXT, deleted_count, 'DELETE'::TEXT;
    
    -- Limpiar datos de campo muy antiguos (eliminación física)
    DELETE FROM driver_field_data 
    WHERE last_verified_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND confidence_score < 3;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN QUERY SELECT 'driver_field_data'::TEXT, deleted_count, 'DELETE'::TEXT;
    
    -- Limpiar tournees muy antiguas (soft delete)
    UPDATE tournees 
    SET deleted_at = NOW() 
    WHERE tournee_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND deleted_at IS NULL;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN QUERY SELECT 'tournees'::TEXT, deleted_count, 'SOFT_DELETE'::TEXT;
    
    -- Limpiar packages muy antiguos (soft delete)
    UPDATE packages 
    SET deleted_at = NOW() 
    WHERE delivery_date < CURRENT_DATE - INTERVAL '1 month' * months_to_keep
    AND deleted_at IS NULL;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN QUERY SELECT 'packages'::TEXT, deleted_count, 'SOFT_DELETE'::TEXT;
    
    -- Refrescar materialized views después de la limpieza
    PERFORM refresh_all_materialized_views();
    
    RAISE NOTICE 'Limpieza agresiva completada. Materialized views refrescadas.';
END;
$$ LANGUAGE plpgsql;

-- Función para gestionar particiones automáticamente
CREATE OR REPLACE FUNCTION manage_performance_partitions()
RETURNS VOID AS $$
DECLARE
    next_month DATE;
    partition_name TEXT;
    partition_start DATE;
    partition_end DATE;
    current_month DATE := DATE_TRUNC('month', CURRENT_DATE)::DATE;
BEGIN
    -- Crear partición para el próximo mes si no existe
    next_month := current_month + INTERVAL '1 month';
    partition_name := 'performance_analytics_' || 
                     TO_CHAR(next_month, 'YYYY_MM');
    
    -- Verificar si la partición ya existe
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON c.relnamespace = n.oid
        WHERE c.relname = partition_name
        AND n.nspname = 'public'
    ) THEN
        partition_start := next_month;
        partition_end := next_month + INTERVAL '1 month';
        
        -- Crear la nueva partición
        EXECUTE format(
            'CREATE TABLE %I PARTITION OF performance_analytics
             FOR VALUES FROM (%L) TO (%L)',
            partition_name, partition_start, partition_end
        );
        
        -- Crear índices para la nueva partición
        EXECUTE format(
            'CREATE INDEX %I ON %I(company_id, driver_id)',
            'idx_' || partition_name || '_company_driver',
            partition_name
        );
        
        RAISE NOTICE 'Nueva partición creada: % para el período % a %', 
                    partition_name, partition_start, partition_end;
    ELSE
        RAISE NOTICE 'La partición % ya existe', partition_name;
    END IF;
    
    -- Eliminar particiones muy antiguas (más de 24 meses)
    FOR partition_name IN
        SELECT c.relname 
        FROM pg_class c
        JOIN pg_namespace n ON c.relnamespace = n.oid
        WHERE c.relname LIKE 'performance_analytics_%'
        AND c.relname != 'performance_analytics_future'
        AND c.relname ~ '^\d{4}_\d{2}$'
    LOOP
        -- Extraer fecha de la partición
        partition_start := TO_DATE(
            REPLACE(partition_name, 'performance_analytics_', ''), 
            'YYYY_MM'
        );
        
        -- Si la partición es más antigua que 24 meses, eliminarla
        IF partition_start < current_month - INTERVAL '24 months' THEN
            EXECUTE format('DROP TABLE %I', partition_name);
            RAISE NOTICE 'Partición antigua eliminada: %', partition_name;
        END IF;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- =====================================================
-- MATERIALIZED VIEWS PARA REPORTES PESADOS
-- =====================================================

-- Materialized View para resumen mensual de empresa
CREATE MATERIALIZED VIEW monthly_company_summary AS
SELECT 
    c.company_id,
    c.name as company_name,
    DATE_TRUNC('month', pa.week_start_date)::DATE as month_start,
    COUNT(DISTINCT pa.driver_id) as active_drivers,
    COUNT(DISTINCT t.vehicle_id) as active_vehicles,
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
    SUM(pa.total_damage_cost) as total_damage_cost,
    AVG(pa.route_optimization_score) as avg_route_optimization,
    COUNT(DISTINCT t.tournee_id) as total_tournees,
    COUNT(DISTINCT CASE WHEN t.tournee_status = 'completed' THEN t.tournee_id END) as completed_tournees
FROM companies c
LEFT JOIN performance_analytics pa ON c.company_id = pa.company_id
LEFT JOIN tournees t ON c.company_id = t.company_id 
    AND DATE_TRUNC('month', t.tournee_date) = DATE_TRUNC('month', pa.week_start_date)
    AND t.deleted_at IS NULL
WHERE c.deleted_at IS NULL
GROUP BY c.company_id, c.name, DATE_TRUNC('month', pa.week_start_date)
ORDER BY c.company_id, month_start DESC;

-- Materialized View para ranking mensual de choferes
CREATE MATERIALIZED VIEW driver_ranking_monthly AS
SELECT 
    u.user_id,
    u.full_name as driver_name,
    c.name as company_name,
    DATE_TRUNC('month', pa.week_start_date)::DATE as month_start,
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
    pa.route_optimization_score,
    pa.efficiency_ratio,
    pa.cost_per_package,
    -- Calcular score de performance (0-100)
    ROUND(
        (COALESCE(pa.delivery_success_rate, 0) * 0.4) +
        (COALESCE(pa.fuel_efficiency / 20, 0) * 0.3) +
        (COALESCE(pa.route_optimization_score, 0) * 0.2) +
        (GREATEST(0, 100 - COALESCE(pa.total_damage_cost, 0)) / 100 * 0.1), 2
    ) as performance_score,
    -- Ranking dentro de la empresa
    ROW_NUMBER() OVER (
        PARTITION BY c.company_id, DATE_TRUNC('month', pa.week_start_date)
        ORDER BY 
            COALESCE(pa.delivery_success_rate, 0) DESC,
            COALESCE(pa.fuel_efficiency, 0) DESC,
            COALESCE(pa.route_optimization_score, 0) DESC
    ) as company_rank
FROM users u
JOIN companies c ON u.company_id = c.company_id
JOIN performance_analytics pa ON u.user_id = pa.driver_id
WHERE u.user_type = 'driver' 
AND u.deleted_at IS NULL
AND c.deleted_at IS NULL
ORDER BY c.company_id, month_start DESC, company_rank ASC;

-- Materialized View para análisis de costos por empresa
CREATE MATERIALIZED VIEW company_cost_analysis AS
SELECT 
    c.company_id,
    c.name as company_name,
    DATE_TRUNC('month', pa.week_start_date)::DATE as month_start,
    -- Costos operativos
    SUM(pa.fuel_cost) as total_fuel_cost,
    SUM(pa.total_damage_cost) as total_damage_cost,
    -- Métricas de eficiencia
    SUM(pa.km_driven) as total_km_driven,
    SUM(pa.total_packages) as total_packages,
    -- Costos por unidad
    ROUND(
        SUM(pa.fuel_cost) / NULLIF(SUM(pa.km_driven), 0), 4
    ) as cost_per_km,
    ROUND(
        (SUM(pa.fuel_cost) + SUM(pa.total_damage_cost)) / 
        NULLIF(SUM(pa.total_packages), 0), 2
    ) as total_cost_per_package,
    -- Análisis de tendencias
    LAG(SUM(pa.fuel_cost), 1) OVER (
        PARTITION BY c.company_id 
        ORDER BY DATE_TRUNC('month', pa.week_start_date)
    ) as prev_month_fuel_cost,
    LAG(SUM(pa.total_damage_cost), 1) OVER (
        PARTITION BY c.company_id 
        ORDER BY DATE_TRUNC('month', pa.week_start_date)
    ) as prev_month_damage_cost
FROM companies c
LEFT JOIN performance_analytics pa ON c.company_id = pa.company_id
WHERE c.deleted_at IS NULL
GROUP BY c.company_id, c.name, DATE_TRUNC('month', pa.week_start_date)
ORDER BY c.company_id, month_start DESC;

-- =====================================================
-- FUNCIONES PARA REFRESH DE MATERIALIZED VIEWS
-- =====================================================

-- Función para refrescar todas las materialized views
CREATE OR REPLACE FUNCTION refresh_all_materialized_views()
RETURNS VOID AS $$
BEGIN
    REFRESH MATERIALIZED VIEW monthly_company_summary;
    REFRESH MATERIALIZED VIEW driver_ranking_monthly;
    REFRESH MATERIALIZED VIEW company_cost_analysis;
    
    RAISE NOTICE 'Todas las materialized views han sido refrescadas';
END;
$$ LANGUAGE plpgsql;

-- Función para refrescar materialized views de un mes específico
CREATE OR REPLACE FUNCTION refresh_monthly_views(target_month DATE)
RETURNS VOID AS $$
BEGIN
    -- Eliminar datos del mes específico
    DELETE FROM monthly_company_summary 
    WHERE month_start = DATE_TRUNC('month', target_month)::DATE;
    
    DELETE FROM driver_ranking_monthly 
    WHERE month_start = DATE_TRUNC('month', target_month)::DATE;
    
    DELETE FROM company_cost_analysis 
    WHERE month_start = DATE_TRUNC('month', target_month)::DATE;
    
    -- Refrescar todas las views
    PERFORM refresh_all_materialized_views();
    
    RAISE NOTICE 'Materialized views refrescadas para el mes: %', target_month;
END;
$$ LANGUAGE plpgsql;
