-- =====================================================
-- MIGRACIÓN: OPTIMIZACIONES DEL SCHEMA
-- Delivery Route Optimizer - Performance Optimizations
-- =====================================================

-- Este archivo aplica todas las optimizaciones de performance
-- incluyendo índices compuestos, particionamiento y materialized views

BEGIN;

-- =====================================================
-- 1. AGREGAR CAMPOS COMPANY_ID FALTANTES
-- =====================================================

-- Agregar company_id a packages si no existe
ALTER TABLE packages 
ADD COLUMN IF NOT EXISTS company_id UUID REFERENCES companies(company_id) ON DELETE CASCADE;

-- Agregar company_id a vehicle_documents si no existe
ALTER TABLE vehicle_documents 
ADD COLUMN IF NOT EXISTS company_id UUID REFERENCES companies(company_id) ON DELETE CASCADE;

-- Actualizar company_id en packages basado en tournees
UPDATE packages 
SET company_id = t.company_id
FROM tournees t
WHERE packages.tournee_id = t.tournee_id
AND packages.company_id IS NULL;

-- Actualizar company_id en vehicle_documents basado en vehicles
UPDATE vehicle_documents 
SET company_id = v.company_id
FROM vehicles v
WHERE vehicle_documents.vehicle_id = v.vehicle_id
AND vehicle_documents.company_id IS NULL;

-- Hacer company_id NOT NULL después de la actualización
ALTER TABLE packages 
ALTER COLUMN company_id SET NOT NULL;

ALTER TABLE vehicle_documents 
ALTER COLUMN company_id SET NOT NULL;

-- =====================================================
-- 2. CREAR ÍNDICES COMPUESTOS MULTI-TENANT
-- =====================================================

-- Índices compuestos para tournees
CREATE INDEX IF NOT EXISTS idx_tournees_company_date_driver 
ON tournees(company_id, tournee_date, driver_id);

CREATE INDEX IF NOT EXISTS idx_tournees_company_status_date 
ON tournees(company_id, tournee_status, tournee_date);

CREATE INDEX IF NOT EXISTS idx_tournees_company_vehicle_date 
ON tournees(company_id, vehicle_id, tournee_date);

-- Índices compuestos para packages
CREATE INDEX IF NOT EXISTS idx_packages_company_status_date 
ON packages(company_id, delivery_status, delivery_date);

CREATE INDEX IF NOT EXISTS idx_packages_company_tournee_date 
ON packages(company_id, tournee_id, delivery_date);

CREATE INDEX IF NOT EXISTS idx_packages_company_failure_reason 
ON packages(company_id, failure_reason, delivery_date);

-- Índices compuestos para vehicle_documents
CREATE INDEX IF NOT EXISTS idx_vehicle_documents_company_status_expiry 
ON vehicle_documents(company_id, document_status, expiry_date);

CREATE INDEX IF NOT EXISTS idx_vehicle_documents_company_type_status 
ON vehicle_documents(company_id, document_type, document_status);

CREATE INDEX IF NOT EXISTS idx_vehicle_documents_company_vehicle_type 
ON vehicle_documents(company_id, vehicle_id, document_type);

-- =====================================================
-- 3. IMPLEMENTAR PARTICIONAMIENTO EN PERFORMANCE_ANALYTICS
-- =====================================================

-- Verificar si la tabla ya está particionada
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON c.relnamespace = n.oid
        WHERE c.relname = 'performance_analytics'
        AND c.relispartition = true
    ) THEN
        -- Crear tabla temporal con la nueva estructura
        CREATE TABLE performance_analytics_new (
            LIKE performance_analytics INCLUDING ALL
        ) PARTITION BY RANGE (week_start_date);
        
        -- Crear particiones por mes
        CREATE TABLE performance_analytics_2024_01 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
        
        CREATE TABLE performance_analytics_2024_02 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-02-01') TO ('2024-03-01');
        
        CREATE TABLE performance_analytics_2024_03 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-03-01') TO ('2024-04-01');
        
        CREATE TABLE performance_analytics_2024_04 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-04-01') TO ('2024-05-01');
        
        CREATE TABLE performance_analytics_2024_05 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-05-01') TO ('2024-06-01');
        
        CREATE TABLE performance_analytics_2024_06 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-06-01') TO ('2024-07-01');
        
        CREATE TABLE performance_analytics_2024_07 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-07-01') TO ('2024-08-01');
        
        CREATE TABLE performance_analytics_2024_08 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-08-01') TO ('2024-09-01');
        
        CREATE TABLE performance_analytics_2024_09 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-09-01') TO ('2024-10-01');
        
        CREATE TABLE performance_analytics_2024_10 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-10-01') TO ('2024-11-01');
        
        CREATE TABLE performance_analytics_2024_11 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-11-01') TO ('2024-12-01');
        
        CREATE TABLE performance_analytics_2024_12 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2024-12-01') TO ('2025-01-01');
        
        -- Particiones para 2025
        CREATE TABLE performance_analytics_2025_01 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');
        
        CREATE TABLE performance_analytics_2025_02 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2025-02-01') TO ('2025-03-01');
        
        CREATE TABLE performance_analytics_2025_03 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2025-03-01') TO ('2025-04-01');
        
        CREATE TABLE performance_analytics_2025_04 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2025-04-01') TO ('2025-05-01');
        
        CREATE TABLE performance_analytics_2025_05 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2025-05-01') TO ('2025-06-01');
        
        CREATE TABLE performance_analytics_2025_06 PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2025-06-01') TO ('2025-07-01');
        
        -- Partición por defecto para fechas futuras
        CREATE TABLE performance_analytics_future PARTITION OF performance_analytics_new
            FOR VALUES FROM ('2025-07-01') TO (MAXVALUE);
        
        -- Copiar datos existentes
        INSERT INTO performance_analytics_new SELECT * FROM performance_analytics;
        
        -- Renombrar tablas
        ALTER TABLE performance_analytics RENAME TO performance_analytics_old;
        ALTER TABLE performance_analytics_new RENAME TO performance_analytics;
        
        -- Crear índices en la nueva tabla particionada
        CREATE INDEX idx_performance_analytics_company_id ON performance_analytics(company_id);
        CREATE INDEX idx_performance_analytics_driver_id ON performance_analytics(driver_id);
        CREATE INDEX idx_performance_analytics_week_start ON performance_analytics(week_start_date);
        CREATE INDEX idx_performance_analytics_week_end ON performance_analytics(week_end_date);
        CREATE INDEX idx_performance_analytics_driver_week ON performance_analytics(driver_id, week_start_date);
        CREATE INDEX idx_performance_analytics_company_week ON performance_analytics(company_id, week_start_date);
        
        -- Crear índices específicos para cada partición
        CREATE INDEX idx_performance_analytics_2024_01_company_driver ON performance_analytics_2024_01(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_02_company_driver ON performance_analytics_2024_02(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_03_company_driver ON performance_analytics_2024_03(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_04_company_driver ON performance_analytics_2024_04(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_05_company_driver ON performance_analytics_2024_05(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_06_company_driver ON performance_analytics_2024_06(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_07_company_driver ON performance_analytics_2024_07(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_08_company_driver ON performance_analytics_2024_08(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_09_company_driver ON performance_analytics_2024_09(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_10_company_driver ON performance_analytics_2024_10(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_11_company_driver ON performance_analytics_2024_11(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2024_12_company_driver ON performance_analytics_2024_12(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2025_01_company_driver ON performance_analytics_2025_01(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2025_02_company_driver ON performance_analytics_2025_02(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2025_03_company_driver ON performance_analytics_2025_03(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2025_04_company_driver ON performance_analytics_2025_04(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2025_05_company_driver ON performance_analytics_2025_05(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_2025_06_company_driver ON performance_analytics_2025_06(company_id, driver_id);
        CREATE INDEX idx_performance_analytics_future_company_driver ON performance_analytics_future(company_id, driver_id);
        
        -- Eliminar tabla antigua
        DROP TABLE performance_analytics_old;
        
        RAISE NOTICE 'Tabla performance_analytics particionada exitosamente';
    ELSE
        RAISE NOTICE 'La tabla performance_analytics ya está particionada';
    END IF;
END $$;

-- =====================================================
-- 4. CREAR MATERIALIZED VIEWS
-- =====================================================

-- Materialized View para resumen mensual de empresa
CREATE MATERIALIZED VIEW IF NOT EXISTS monthly_company_summary AS
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
CREATE MATERIALIZED VIEW IF NOT EXISTS driver_ranking_monthly AS
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
CREATE MATERIALIZED VIEW IF NOT EXISTS company_cost_analysis AS
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
-- 5. CREAR FUNCIONES DE MANTENIMIENTO
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
-- 6. VERIFICAR MIGRACIÓN
-- =====================================================

-- Verificar que todos los índices compuestos se crearon
DO $$
DECLARE
    missing_indexes TEXT[] := ARRAY[]::TEXT[];
BEGIN
    -- Verificar índices de tournees
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_tournees_company_date_driver') THEN
        missing_indexes := array_append(missing_indexes, 'idx_tournees_company_date_driver');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_tournees_company_status_date') THEN
        missing_indexes := array_append(missing_indexes, 'idx_tournees_company_status_date');
    END IF;
    
    -- Verificar índices de packages
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_packages_company_status_date') THEN
        missing_indexes := array_append(missing_indexes, 'idx_packages_company_status_date');
    END IF;
    
    -- Verificar índices de vehicle_documents
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_vehicle_documents_company_status_expiry') THEN
        missing_indexes := array_append(missing_indexes, 'idx_vehicle_documents_company_status_expiry');
    END IF;
    
    -- Reportar resultado
    IF array_length(missing_indexes, 1) > 0 THEN
        RAISE NOTICE 'ERROR: Faltan los siguientes índices: %', array_to_string(missing_indexes, ', ');
        RAISE EXCEPTION 'Migración de índices incompleta';
    ELSE
        RAISE NOTICE 'SUCCESS: Todos los índices compuestos se crearon correctamente';
    END IF;
END $$;

-- Verificar particionamiento
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON c.relnamespace = n.oid
        WHERE c.relname = 'performance_analytics'
        AND c.relispartition = true
    ) THEN
        RAISE NOTICE 'SUCCESS: Tabla performance_analytics particionada correctamente';
    ELSE
        RAISE NOTICE 'WARNING: Tabla performance_analytics no está particionada';
    END IF;
END $$;

-- Verificar materialized views
DO $$
DECLARE
    missing_views TEXT[] := ARRAY[]::TEXT[];
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_matviews WHERE matviewname = 'monthly_company_summary') THEN
        missing_views := array_append(missing_views, 'monthly_company_summary');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_matviews WHERE matviewname = 'driver_ranking_monthly') THEN
        missing_views := array_append(missing_views, 'driver_ranking_monthly');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_matviews WHERE matviewname = 'company_cost_analysis') THEN
        missing_views := array_append(missing_views, 'company_cost_analysis');
    END IF;
    
    IF array_length(missing_views, 1) > 0 THEN
        RAISE NOTICE 'ERROR: Faltan las siguientes materialized views: %', array_to_string(missing_views, ', ');
        RAISE EXCEPTION 'Migración de materialized views incompleta';
    ELSE
        RAISE NOTICE 'SUCCESS: Todas las materialized views se crearon correctamente';
    END IF;
END $$;

-- =====================================================
-- 7. INFORMACIÓN DE MIGRACIÓN
-- =====================================================

RAISE NOTICE '=====================================================';
RAISE NOTICE 'MIGRACIÓN DE OPTIMIZACIONES COMPLETADA EXITOSAMENTE';
RAISE NOTICE '=====================================================';
RAISE NOTICE 'Optimizaciones aplicadas:';
RAISE NOTICE '✓ Índices compuestos multi-tenant';
RAISE NOTICE '✓ Particionamiento por mes en performance_analytics';
RAISE NOTICE '✓ Materialized views para reportes pesados';
RAISE NOTICE '✓ Funciones de mantenimiento automático';
RAISE NOTICE '=====================================================';
RAISE NOTICE 'Próximos pasos:';
RAISE NOTICE '1. Configurar cron job para refresh de materialized views';
RAISE NOTICE '2. Configurar cron job para gestión automática de particiones';
RAISE NOTICE '3. Configurar cron job para cleanup automático de datos';
RAISE NOTICE '4. Monitorear performance de queries con EXPLAIN ANALYZE';
RAISE NOTICE '=====================================================';

COMMIT;
