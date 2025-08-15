-- =====================================================
-- MIGRACIÓN: INTEGRACIONES CON APIs EXTERNAS
-- Delivery Route Optimizer - API Integration Support
-- =====================================================

-- Este archivo permite migrar una base de datos existente
-- para agregar soporte completo de integraciones con APIs

BEGIN;

-- =====================================================
-- 1. CREAR ENUM Y TABLA API_INTEGRATIONS
-- =====================================================

-- Crear enum para sync_status si no existe
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'sync_status') THEN
        CREATE TYPE sync_status AS ENUM ('active', 'error', 'disabled', 'syncing');
    END IF;
END $$;

-- Crear tabla api_integrations si no existe
CREATE TABLE IF NOT EXISTS api_integrations (
    integration_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    
    -- Información del proveedor
    provider_name VARCHAR(100) NOT NULL,
    provider_display_name VARCHAR(255),
    api_version VARCHAR(20),
    
    -- Credenciales y configuración
    api_credentials JSONB NOT NULL, -- Encriptado con pgcrypto
    api_endpoint TEXT,
    webhook_url TEXT,
    
    -- Estado de sincronización
    sync_status sync_status NOT NULL DEFAULT 'active',
    last_sync_date TIMESTAMP WITH TIME ZONE,
    last_successful_sync TIMESTAMP WITH TIME ZONE,
    consecutive_errors INTEGER DEFAULT 0,
    
    -- Límites y frecuencia
    daily_sync_limit INTEGER DEFAULT 1000,
    sync_frequency_hours INTEGER DEFAULT 24,
    max_retry_attempts INTEGER DEFAULT 3,
    
    -- Configuración específica del proveedor
    provider_config JSONB DEFAULT '{}',
    field_mappings JSONB DEFAULT '{}', -- Mapeo de campos entre sistemas
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT unique_provider_per_company UNIQUE (company_id, provider_name),
    CONSTRAINT valid_sync_frequency CHECK (sync_frequency_hours >= 1),
    CONSTRAINT valid_daily_limit CHECK (daily_sync_limit > 0),
    CONSTRAINT valid_retry_attempts CHECK (max_retry_attempts >= 0)
);

-- =====================================================
-- 2. CREAR TABLA SYNC_LOG
-- =====================================================

CREATE TABLE IF NOT EXISTS sync_log (
    sync_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    integration_id UUID NOT NULL REFERENCES api_integrations(integration_id) ON DELETE CASCADE,
    
    -- Información de la sincronización
    sync_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    sync_type VARCHAR(50) NOT NULL, -- 'full_sync', 'incremental', 'webhook'
    sync_direction VARCHAR(20) NOT NULL, -- 'inbound', 'outbound', 'bidirectional'
    
    -- Métricas de la sincronización
    records_processed INTEGER NOT NULL DEFAULT 0,
    records_created INTEGER DEFAULT 0,
    records_updated INTEGER DEFAULT 0,
    records_deleted INTEGER DEFAULT 0,
    records_failed INTEGER DEFAULT 0,
    errors_count INTEGER NOT NULL DEFAULT 0,
    
    -- Performance y duración
    sync_duration_seconds INTEGER,
    sync_start_time TIMESTAMP WITH TIME ZONE,
    sync_end_time TIMESTAMP WITH TIME ZONE,
    
    -- Detalles de errores y estado
    error_details JSONB DEFAULT '{}',
    sync_status VARCHAR(20) NOT NULL DEFAULT 'completed', -- 'completed', 'failed', 'partial'
    retry_count INTEGER DEFAULT 0,
    
    -- Metadatos adicionales
    api_response_code INTEGER,
    api_response_time_ms INTEGER,
    data_size_bytes BIGINT,
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT valid_sync_duration CHECK (sync_duration_seconds >= 0),
    CONSTRAINT valid_records_count CHECK (records_processed >= 0),
    CONSTRAINT valid_errors_count CHECK (errors_count >= 0),
    CONSTRAINT valid_sync_times CHECK (
        (sync_start_time IS NULL AND sync_end_time IS NULL) OR
        (sync_start_time IS NOT NULL AND sync_end_time IS NOT NULL AND sync_end_time >= sync_start_time)
    )
);

-- =====================================================
-- 3. AGREGAR CAMPOS DE ORIGEN A TABLAS EXISTENTES
-- =====================================================

-- Agregar campos a tournees
ALTER TABLE tournees 
ADD COLUMN IF NOT EXISTS tournee_origin VARCHAR(50) DEFAULT 'manual';

ALTER TABLE tournees 
ADD COLUMN IF NOT EXISTS external_tournee_id VARCHAR(100);

ALTER TABLE tournees 
ADD COLUMN IF NOT EXISTS integration_id UUID REFERENCES api_integrations(integration_id) ON DELETE SET NULL;

-- Agregar campos a packages
ALTER TABLE packages 
ADD COLUMN IF NOT EXISTS package_origin VARCHAR(50) DEFAULT 'manual';

ALTER TABLE packages 
ADD COLUMN IF NOT EXISTS external_package_id VARCHAR(100);

ALTER TABLE packages 
ADD COLUMN IF NOT EXISTS integration_id UUID REFERENCES api_integrations(integration_id) ON DELETE SET NULL;

-- =====================================================
-- 4. CREAR ÍNDICES PARA NUEVAS TABLAS
-- =====================================================

-- Índices para api_integrations
CREATE INDEX IF NOT EXISTS idx_api_integrations_company_id ON api_integrations(company_id);
CREATE INDEX IF NOT EXISTS idx_api_integrations_provider_name ON api_integrations(provider_name);
CREATE INDEX IF NOT EXISTS idx_api_integrations_sync_status ON api_integrations(sync_status);
CREATE INDEX IF NOT EXISTS idx_api_integrations_last_sync ON api_integrations(last_sync_date);
CREATE INDEX IF NOT EXISTS idx_api_integrations_company_provider ON api_integrations(company_id, provider_name);
CREATE INDEX IF NOT EXISTS idx_api_integrations_status_provider ON api_integrations(sync_status, provider_name);

-- Índices para sync_log
CREATE INDEX IF NOT EXISTS idx_sync_log_integration_id ON sync_log(integration_id);
CREATE INDEX IF NOT EXISTS idx_sync_log_sync_date ON sync_log(sync_date);
CREATE INDEX IF NOT EXISTS idx_sync_log_sync_status ON sync_log(sync_status);
CREATE INDEX IF NOT EXISTS idx_sync_log_sync_type ON sync_log(sync_type);
CREATE INDEX IF NOT EXISTS idx_sync_log_errors_count ON sync_log(errors_count);
CREATE INDEX IF NOT EXISTS idx_sync_log_company_date ON sync_log(company_id, sync_date);
CREATE INDEX IF NOT EXISTS idx_sync_log_integration_date ON sync_log(integration_id, sync_date);
CREATE INDEX IF NOT EXISTS idx_sync_log_status_date ON sync_log(sync_status, sync_date);

-- Índices para campos de origen en tournees
CREATE INDEX IF NOT EXISTS idx_tournees_origin ON tournees(tournee_origin);
CREATE INDEX IF NOT EXISTS idx_tournees_external_id ON tournees(external_tournee_id);
CREATE INDEX IF NOT EXISTS idx_tournees_integration_id ON tournees(integration_id);

-- Índices para campos de origen en packages
CREATE INDEX IF NOT EXISTS idx_packages_origin ON packages(package_origin);
CREATE INDEX IF NOT EXISTS idx_packages_external_id ON packages(external_package_id);
CREATE INDEX IF NOT EXISTS idx_packages_integration_id ON packages(integration_id);

-- =====================================================
-- 5. AGREGAR CONSTRAINTS DE VALIDACIÓN
-- =====================================================

-- Constraints para tournees
ALTER TABLE tournees 
ADD CONSTRAINT IF NOT EXISTS valid_tournee_origin 
CHECK (tournee_origin IN ('manual', 'api_sync', 'webhook'));

ALTER TABLE tournees 
ADD CONSTRAINT IF NOT EXISTS valid_external_tournee_id 
CHECK (
    (tournee_origin = 'manual' AND external_tournee_id IS NULL) OR
    (tournee_origin IN ('api_sync', 'webhook') AND external_tournee_id IS NOT NULL)
);

-- Constraints para packages
ALTER TABLE packages 
ADD CONSTRAINT IF NOT EXISTS valid_package_origin 
CHECK (package_origin IN ('manual', 'api_sync', 'webhook'));

ALTER TABLE packages 
ADD CONSTRAINT IF NOT EXISTS valid_external_package_id 
CHECK (
    (package_origin = 'manual' AND external_package_id IS NULL) OR
    (package_origin IN ('api_sync', 'webhook') AND external_package_id IS NOT NULL)
);

-- =====================================================
-- 6. CREAR FUNCIONES DE GESTIÓN DE APIs
-- =====================================================

-- Función para marcar tournée como sincronizada desde API externa
CREATE OR REPLACE FUNCTION mark_tournee_as_api_synced(
    tournee_uuid UUID,
    api_integration_uuid UUID,
    external_tournee_id_param VARCHAR(100),
    sync_metadata JSONB DEFAULT '{}'
)
RETURNS BOOLEAN AS $$
DECLARE
    company_uuid UUID;
    integration_exists BOOLEAN;
BEGIN
    -- Verificar que la tournée existe
    SELECT company_id INTO company_uuid
    FROM tournees
    WHERE tournee_id = tournee_uuid
    AND deleted_at IS NULL;
    
    IF company_uuid IS NULL THEN
        RAISE EXCEPTION 'Tournée no encontrada: %', tournee_uuid;
    END IF;
    
    -- Verificar que la integración existe y pertenece a la misma empresa
    SELECT EXISTS(
        SELECT 1 FROM api_integrations
        WHERE integration_id = api_integration_uuid
        AND company_id = company_uuid
        AND deleted_at IS NULL
    ) INTO integration_exists;
    
    IF NOT integration_exists THEN
        RAISE EXCEPTION 'Integración no válida para esta empresa: %', api_integration_uuid;
    END IF;
    
    -- Actualizar la tournée
    UPDATE tournees SET
        tournee_origin = 'api_sync',
        external_tournee_id = external_tournee_id_param,
        integration_id = api_integration_uuid,
        updated_at = NOW()
    WHERE tournee_id = tournee_uuid;
    
    -- Registrar en el log de sincronización
    INSERT INTO sync_log (
        company_id,
        integration_id,
        sync_type,
        sync_direction,
        records_processed,
        records_updated,
        sync_status,
        sync_duration_seconds,
        sync_start_time,
        sync_end_time
    ) VALUES (
        company_uuid,
        api_integration_uuid,
        'incremental',
        'inbound',
        1,
        1,
        'completed',
        EXTRACT(EPOCH FROM (NOW() - NOW())),
        NOW(),
        NOW()
    );
    
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Función para marcar paquete como sincronizado desde API externa
CREATE OR REPLACE FUNCTION mark_package_as_api_synced(
    package_uuid UUID,
    api_integration_uuid UUID,
    external_package_id_param VARCHAR(100),
    sync_metadata JSONB DEFAULT '{}'
)
RETURNS BOOLEAN AS $$
DECLARE
    company_uuid UUID;
    integration_exists BOOLEAN;
BEGIN
    -- Verificar que el paquete existe
    SELECT company_id INTO company_uuid
    FROM packages
    WHERE package_id = package_uuid
    AND deleted_at IS NULL;
    
    IF company_uuid IS NULL THEN
        RAISE EXCEPTION 'Paquete no encontrado: %', package_uuid;
    END IF;
    
    -- Verificar que la integración existe y pertenece a la misma empresa
    SELECT EXISTS(
        SELECT 1 FROM api_integrations
        WHERE integration_id = api_integration_uuid
        AND company_id = company_uuid
        AND deleted_at IS NULL
    ) INTO integration_exists;
    
    IF NOT integration_exists THEN
        RAISE EXCEPTION 'Integración no válida para esta empresa: %', api_integration_uuid;
    END IF;
    
    -- Actualizar el paquete
    UPDATE packages SET
        package_origin = 'api_sync',
        external_package_id = external_package_id_param,
        integration_id = api_integration_uuid,
        updated_at = NOW()
    WHERE package_id = package_uuid;
    
    -- Registrar en el log de sincronización
    INSERT INTO sync_log (
        company_id,
        integration_id,
        sync_type,
        sync_direction,
        records_processed,
        records_updated,
        sync_status,
        sync_duration_seconds,
        sync_start_time,
        sync_end_time
    ) VALUES (
        company_uuid,
        api_integration_uuid,
        'incremental',
        'inbound',
        1,
        1,
        'completed',
        EXTRACT(EPOCH FROM (NOW() - NOW())),
        NOW(),
        NOW()
    );
    
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Función para crear nueva integración con API
CREATE OR REPLACE FUNCTION create_api_integration(
    company_uuid UUID,
    provider_name_param VARCHAR(100),
    provider_display_name_param VARCHAR(255),
    api_credentials_param JSONB,
    api_endpoint_param TEXT DEFAULT NULL,
    webhook_url_param TEXT DEFAULT NULL,
    daily_sync_limit_param INTEGER DEFAULT 1000,
    sync_frequency_hours_param INTEGER DEFAULT 24
)
RETURNS UUID AS $$
DECLARE
    new_integration_id UUID;
BEGIN
    -- Verificar que la empresa existe
    IF NOT EXISTS (
        SELECT 1 FROM companies 
        WHERE company_id = company_uuid 
        AND deleted_at IS NULL
    ) THEN
        RAISE EXCEPTION 'Empresa no encontrada: %', company_uuid;
    END IF;
    
    -- Verificar que no existe ya una integración para este proveedor
    IF EXISTS (
        SELECT 1 FROM api_integrations
        WHERE company_id = company_uuid
        AND provider_name = provider_name_param
        AND deleted_at IS NULL
    ) THEN
        RAISE EXCEPTION 'Ya existe una integración para el proveedor % en esta empresa', provider_name_param;
    END IF;
    
    -- Crear la nueva integración
    INSERT INTO api_integrations (
        company_id,
        provider_name,
        provider_display_name,
        api_credentials,
        api_endpoint,
        webhook_url,
        daily_sync_limit,
        sync_frequency_hours
    ) VALUES (
        company_uuid,
        provider_name_param,
        provider_display_name_param,
        api_credentials_param,
        api_endpoint_param,
        webhook_url_param,
        daily_sync_limit_param,
        sync_frequency_hours_param
    ) RETURNING integration_id INTO new_integration_id;
    
    RAISE NOTICE 'Nueva integración creada para %: %', provider_name_param, new_integration_id;
    
    RETURN new_integration_id;
END;
$$ LANGUAGE plpgsql;

-- =====================================================
-- 7. VERIFICAR MIGRACIÓN
-- =====================================================

-- Verificar que todas las tablas se crearon correctamente
DO $$
DECLARE
    missing_tables TEXT[] := ARRAY[]::TEXT[];
    missing_columns TEXT[] := ARRAY[]::TEXT[];
BEGIN
    -- Verificar tablas
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'api_integrations') THEN
        missing_tables := array_append(missing_tables, 'api_integrations');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sync_log') THEN
        missing_tables := array_append(missing_tables, 'sync_log');
    END IF;
    
    -- Verificar campos en tournees
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'tournees' AND column_name = 'tournee_origin') THEN
        missing_columns := array_append(missing_columns, 'tournees.tournee_origin');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'tournees' AND column_name = 'external_tournee_id') THEN
        missing_columns := array_append(missing_columns, 'tournees.external_tournee_id');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'tournees' AND column_name = 'integration_id') THEN
        missing_columns := array_append(missing_columns, 'tournees.integration_id');
    END IF;
    
    -- Verificar campos en packages
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'packages' AND column_name = 'package_origin') THEN
        missing_columns := array_append(missing_columns, 'packages.package_origin');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'packages' AND column_name = 'external_package_id') THEN
        missing_columns := array_append(missing_columns, 'packages.external_package_id');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'packages' AND column_name = 'integration_id') THEN
        missing_columns := array_append(missing_columns, 'packages.integration_id');
    END IF;
    
    -- Reportar resultado
    IF array_length(missing_tables, 1) > 0 THEN
        RAISE NOTICE 'ERROR: Faltan las siguientes tablas: %', array_to_string(missing_tables, ', ');
        RAISE EXCEPTION 'Migración de tablas incompleta';
    END IF;
    
    IF array_length(missing_columns, 1) > 0 THEN
        RAISE NOTICE 'ERROR: Faltan las siguientes columnas: %', array_to_string(missing_columns, ', ');
        RAISE EXCEPTION 'Migración de columnas incompleta';
    END IF;
    
    RAISE NOTICE 'SUCCESS: Todas las tablas y columnas se crearon correctamente';
END $$;

-- =====================================================
-- 8. INFORMACIÓN DE MIGRACIÓN
-- =====================================================

RAISE NOTICE '=====================================================';
RAISE NOTICE 'MIGRACIÓN DE INTEGRACIONES CON APIs COMPLETADA';
RAISE NOTICE '=====================================================';
RAISE NOTICE 'Nuevas funcionalidades agregadas:';
RAISE NOTICE '✓ Tabla api_integrations para gestionar proveedores';
RAISE NOTICE '✓ Tabla sync_log para tracking de sincronizaciones';
RAISE NOTICE '✓ Campos de origen en tournees y packages';
RAISE NOTICE '✓ Funciones para marcar datos como sincronizados';
RAISE NOTICE '✓ Función para crear nuevas integraciones';
RAISE NOTICE '=====================================================';
RAISE NOTICE 'Próximos pasos:';
RAISE NOTICE '1. Crear integraciones con proveedores específicos';
RAISE NOTICE '2. Implementar lógica de sincronización en tu aplicación';
RAISE NOTICE '3. Configurar webhooks para sincronización en tiempo real';
RAISE NOTICE '4. Monitorear logs de sincronización';
RAISE NOTICE '=====================================================';
RAISE NOTICE 'Ejemplos de uso:';
RAISE NOTICE '-- Crear integración con Colis Privé';
RAISE NOTICE 'SELECT create_api_integration(';
RAISE NOTICE '    company_uuid,';
RAISE NOTICE '    ''colis_prive'',';
RAISE NOTICE '    ''Colis Privé'',';
RAISE NOTICE '    ''{"api_key": "your_key", "secret": "your_secret"}''';
RAISE NOTICE ');';
RAISE NOTICE '=====================================================';

COMMIT;
