-- =====================================================
-- FUNCIONES PARA GESTIÓN DE INTEGRACIONES CON APIs
-- Delivery Route Optimizer - API Integration Functions
-- =====================================================

-- =====================================================
-- FUNCIONES PARA MARCAR ORIGEN DE DATOS
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

-- =====================================================
-- FUNCIONES PARA LOGGING DE SINCRONIZACIÓN
-- =====================================================

-- Función para registrar sincronización masiva
CREATE OR REPLACE FUNCTION log_bulk_api_sync(
    api_integration_uuid UUID,
    sync_type_param VARCHAR(50),
    sync_direction_param VARCHAR(20),
    records_processed_param INTEGER,
    records_created_param INTEGER DEFAULT 0,
    records_updated_param INTEGER DEFAULT 0,
    records_deleted_param INTEGER DEFAULT 0,
    records_failed_param INTEGER DEFAULT 0,
    errors_count_param INTEGER DEFAULT 0,
    sync_duration_seconds_param INTEGER DEFAULT 0,
    error_details_param JSONB DEFAULT '{}',
    sync_metadata JSONB DEFAULT '{}'
)
RETURNS UUID AS $$
DECLARE
    company_uuid UUID;
    new_sync_id UUID;
    sync_start_time TIMESTAMP WITH TIME ZONE;
    sync_end_time TIMESTAMP WITH TIME ZONE;
BEGIN
    -- Obtener company_id de la integración
    SELECT company_id INTO company_uuid
    FROM api_integrations
    WHERE integration_id = api_integration_uuid
    AND deleted_at IS NULL;
    
    IF company_uuid IS NULL THEN
        RAISE EXCEPTION 'Integración no encontrada: %', api_integration_uuid;
    END IF;
    
    -- Calcular tiempos de sincronización
    sync_start_time := NOW() - INTERVAL '1 second' * sync_duration_seconds_param;
    sync_end_time := NOW();
    
    -- Registrar la sincronización
    INSERT INTO sync_log (
        company_id,
        integration_id,
        sync_type,
        sync_direction,
        records_processed,
        records_created,
        records_updated,
        records_deleted,
        records_failed,
        errors_count,
        sync_duration_seconds,
        sync_start_time,
        sync_end_time,
        error_details,
        sync_status
    ) VALUES (
        company_uuid,
        api_integration_uuid,
        sync_type_param,
        sync_direction_param,
        records_processed_param,
        records_created_param,
        records_updated_param,
        records_deleted_param,
        records_failed_param,
        errors_count_param,
        sync_duration_seconds_param,
        sync_start_time,
        sync_end_time,
        error_details_param,
        CASE 
            WHEN errors_count_param = 0 THEN 'completed'
            WHEN errors_count_param < records_processed_param THEN 'partial'
            ELSE 'failed'
        END
    ) RETURNING sync_id INTO new_sync_id;
    
    -- Actualizar estadísticas de la integración
    UPDATE api_integrations SET
        last_sync_date = NOW(),
        consecutive_errors = CASE 
            WHEN errors_count_param = 0 THEN 0
            ELSE consecutive_errors + 1
        END,
        last_successful_sync = CASE 
            WHEN errors_count_param = 0 THEN NOW()
            ELSE last_successful_sync
        END,
        sync_status = CASE 
            WHEN errors_count_param = 0 THEN 'active'
            WHEN consecutive_errors >= max_retry_attempts THEN 'error'
            ELSE 'active'
        END,
        updated_at = NOW()
    WHERE integration_id = api_integration_uuid;
    
    RETURN new_sync_id;
END;
$$ LANGUAGE plpgsql;

-- =====================================================
-- FUNCIONES PARA CONSULTAS Y ESTADÍSTICAS
-- =====================================================

-- Función para obtener estadísticas de sincronización por empresa
CREATE OR REPLACE FUNCTION get_api_sync_stats(company_uuid UUID, days_back INTEGER DEFAULT 30)
RETURNS TABLE(
    provider_name VARCHAR(100),
    total_syncs BIGINT,
    successful_syncs BIGINT,
    failed_syncs BIGINT,
    total_records_processed BIGINT,
    avg_sync_duration DECIMAL,
    last_sync_date TIMESTAMP WITH TIME ZONE,
    sync_status sync_status
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        ai.provider_name,
        COUNT(sl.sync_id)::BIGINT as total_syncs,
        COUNT(CASE WHEN sl.sync_status = 'completed' THEN 1 END)::BIGINT as successful_syncs,
        COUNT(CASE WHEN sl.sync_status IN ('failed', 'partial') THEN 1 END)::BIGINT as failed_syncs,
        COALESCE(SUM(sl.records_processed), 0)::BIGINT as total_records_processed,
        ROUND(AVG(sl.sync_duration_seconds), 2) as avg_sync_duration,
        MAX(sl.sync_date) as last_sync_date,
        ai.sync_status
    FROM api_integrations ai
    LEFT JOIN sync_log sl ON ai.integration_id = sl.integration_id
        AND sl.sync_date >= CURRENT_DATE - INTERVAL '1 day' * days_back
    WHERE ai.company_id = company_uuid
    AND ai.deleted_at IS NULL
    GROUP BY ai.provider_name, ai.sync_status
    ORDER BY ai.provider_name;
END;
$$ LANGUAGE plpgsql;

-- Función para obtener tournées sincronizadas desde APIs
CREATE OR REPLACE FUNCTION get_api_synced_tournees(
    company_uuid UUID,
    provider_name_param VARCHAR(100) DEFAULT NULL,
    date_from DATE DEFAULT NULL,
    date_to DATE DEFAULT NULL
)
RETURNS TABLE(
    tournee_id UUID,
    tournee_number VARCHAR(50),
    tournee_date DATE,
    tournee_status tournee_status,
    provider_name VARCHAR(100),
    external_tournee_id VARCHAR(100),
    sync_date TIMESTAMP WITH TIME ZONE,
    driver_name VARCHAR(255),
    vehicle_plate VARCHAR(20),
    total_packages INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        t.tournee_id,
        t.tournee_number,
        t.tournee_date,
        t.tournee_status,
        ai.provider_name,
        t.external_tournee_id,
        ai.last_sync_date as sync_date,
        u.full_name as driver_name,
        v.license_plate as vehicle_plate,
        COUNT(p.package_id)::INTEGER as total_packages
    FROM tournees t
    JOIN api_integrations ai ON t.integration_id = ai.integration_id
    JOIN users u ON t.driver_id = u.user_id
    JOIN vehicles v ON t.vehicle_id = v.vehicle_id
    LEFT JOIN packages p ON t.tournee_id = p.tournee_id AND p.deleted_at IS NULL
    WHERE t.company_id = company_uuid
    AND t.tournee_origin = 'api_sync'
    AND t.deleted_at IS NULL
    AND (provider_name_param IS NULL OR ai.provider_name = provider_name_param)
    AND (date_from IS NULL OR t.tournee_date >= date_from)
    AND (date_to IS NULL OR t.tournee_date <= date_to)
    GROUP BY t.tournee_id, t.tournee_number, t.tournee_date, t.tournee_status,
             ai.provider_name, t.external_tournee_id, ai.last_sync_date,
             u.full_name, v.license_plate
    ORDER BY t.tournee_date DESC, t.tournee_number;
END;
$$ LANGUAGE plpgsql;

-- Función para obtener paquetes sincronizados desde APIs
CREATE OR REPLACE FUNCTION get_api_synced_packages(
    company_uuid UUID,
    provider_name_param VARCHAR(100) DEFAULT NULL,
    date_from DATE DEFAULT NULL,
    date_to DATE DEFAULT NULL
)
RETURNS TABLE(
    package_id UUID,
    tracking_number VARCHAR(100),
    external_package_id VARCHAR(100),
    delivery_status delivery_status,
    delivery_date DATE,
    provider_name VARCHAR(100),
    sync_date TIMESTAMP WITH TIME ZONE,
    tournee_number VARCHAR(50),
    recipient_name VARCHAR(255),
    delivery_address TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        p.package_id,
        p.tracking_number,
        p.external_package_id,
        p.delivery_status,
        p.delivery_date,
        ai.provider_name,
        ai.last_sync_date as sync_date,
        t.tournee_number,
        p.recipient_name,
        p.delivery_address
    FROM packages p
    JOIN api_integrations ai ON p.integration_id = ai.integration_id
    LEFT JOIN tournees t ON p.tournee_id = t.tournee_id
    WHERE p.company_id = company_uuid
    AND p.package_origin = 'api_sync'
    AND p.deleted_at IS NULL
    AND (provider_name_param IS NULL OR ai.provider_name = provider_name_param)
    AND (date_from IS NULL OR p.delivery_date >= date_from)
    AND (date_to IS NULL OR p.delivery_date <= date_to)
    ORDER BY p.delivery_date DESC, p.tracking_number;
END;
$$ LANGUAGE plpgsql;

-- =====================================================
-- FUNCIONES PARA GESTIÓN DE INTEGRACIONES
-- =====================================================

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

-- Función para actualizar credenciales de API
CREATE OR REPLACE FUNCTION update_api_credentials(
    integration_uuid UUID,
    new_credentials JSONB,
    new_endpoint TEXT DEFAULT NULL,
    new_webhook_url TEXT DEFAULT NULL
)
RETURNS BOOLEAN AS $$
BEGIN
    UPDATE api_integrations SET
        api_credentials = new_credentials,
        api_endpoint = COALESCE(new_endpoint, api_endpoint),
        webhook_url = COALESCE(new_webhook_url, webhook_url),
        updated_at = NOW()
    WHERE integration_id = integration_uuid
    AND deleted_at IS NULL;
    
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Integración no encontrada: %', integration_uuid;
    END IF;
    
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Función para deshabilitar integración
CREATE OR REPLACE FUNCTION disable_api_integration(integration_uuid UUID)
RETURNS BOOLEAN AS $$
BEGIN
    UPDATE api_integrations SET
        sync_status = 'disabled',
        updated_at = NOW()
    WHERE integration_id = integration_uuid
    AND deleted_at IS NULL;
    
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Integración no encontrada: %', integration_uuid;
    END IF;
    
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Función para habilitar integración
CREATE OR REPLACE FUNCTION enable_api_integration(integration_uuid UUID)
RETURNS BOOLEAN AS $$
BEGIN
    UPDATE api_integrations SET
        sync_status = 'active',
        consecutive_errors = 0,
        updated_at = NOW()
    WHERE integration_id = integration_uuid
    AND deleted_at IS NULL;
    
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Integración no encontrada: %', integration_uuid;
    END IF;
    
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- =====================================================
-- FUNCIONES PARA MONITOREO Y ALERTAS
-- =====================================================

-- Función para detectar integraciones con problemas
CREATE OR REPLACE FUNCTION detect_api_integration_issues()
RETURNS TABLE(
    integration_id UUID,
    company_name VARCHAR(255),
    provider_name VARCHAR(100),
    issue_type VARCHAR(50),
    issue_description TEXT,
    last_sync_date TIMESTAMP WITH TIME ZONE,
    consecutive_errors INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        ai.integration_id,
        c.name as company_name,
        ai.provider_name,
        CASE 
            WHEN ai.consecutive_errors >= ai.max_retry_attempts THEN 'max_retries_exceeded'
            WHEN ai.last_sync_date IS NULL THEN 'never_synced'
            WHEN ai.last_sync_date < NOW() - INTERVAL '1 day' * (ai.sync_frequency_hours / 24) THEN 'sync_overdue'
            WHEN ai.consecutive_errors > 0 THEN 'consecutive_errors'
            ELSE 'no_issues'
        END as issue_type,
        CASE 
            WHEN ai.consecutive_errors >= ai.max_retry_attempts THEN 
                'Integración excedió el máximo de intentos de reintento (' || ai.max_retry_attempts || ')'
            WHEN ai.last_sync_date IS NULL THEN 
                'Integración nunca ha sincronizado'
            WHEN ai.last_sync_date < NOW() - INTERVAL '1 day' * (ai.sync_frequency_hours / 24) THEN 
                'Sincronización atrasada por ' || 
                EXTRACT(EPOCH FROM (NOW() - ai.last_sync_date)) / 3600 || ' horas'
            WHEN ai.consecutive_errors > 0 THEN 
                'Integración tiene ' || ai.consecutive_errors || ' errores consecutivos'
            ELSE 'Integración funcionando correctamente'
        END as issue_description,
        ai.last_sync_date,
        ai.consecutive_errors
    FROM api_integrations ai
    JOIN companies c ON ai.company_id = c.company_id
    WHERE ai.sync_status = 'active'
    AND ai.deleted_at IS NULL
    AND c.deleted_at IS NULL
    AND (
        ai.consecutive_errors >= ai.max_retry_attempts OR
        ai.last_sync_date IS NULL OR
        ai.last_sync_date < NOW() - INTERVAL '1 day' * (ai.sync_frequency_hours / 24) OR
        ai.consecutive_errors > 0
    )
    ORDER BY ai.consecutive_errors DESC, ai.last_sync_date ASC;
END;
$$ LANGUAGE plpgsql;

-- Función para obtener resumen de integraciones por empresa
CREATE OR REPLACE FUNCTION get_company_api_integrations_summary(company_uuid UUID)
RETURNS TABLE(
    total_integrations BIGINT,
    active_integrations BIGINT,
    error_integrations BIGINT,
    disabled_integrations BIGINT,
    last_sync_overall TIMESTAMP WITH TIME ZONE,
    total_records_synced_today BIGINT,
    total_records_synced_week BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(ai.integration_id)::BIGINT as total_integrations,
        COUNT(CASE WHEN ai.sync_status = 'active' THEN 1 END)::BIGINT as active_integrations,
        COUNT(CASE WHEN ai.sync_status = 'error' THEN 1 END)::BIGINT as error_integrations,
        COUNT(CASE WHEN ai.sync_status = 'disabled' THEN 1 END)::BIGINT as disabled_integrations,
        MAX(ai.last_sync_date) as last_sync_overall,
        COALESCE(SUM(CASE WHEN sl.sync_date >= CURRENT_DATE THEN sl.records_processed ELSE 0 END), 0)::BIGINT as total_records_synced_today,
        COALESCE(SUM(CASE WHEN sl.sync_date >= CURRENT_DATE - INTERVAL '7 days' THEN sl.records_processed ELSE 0 END), 0)::BIGINT as total_records_synced_week
    FROM api_integrations ai
    LEFT JOIN sync_log sl ON ai.integration_id = sl.integration_id
    WHERE ai.company_id = company_uuid
    AND ai.deleted_at IS NULL;
END;
$$ LANGUAGE plpgsql;
