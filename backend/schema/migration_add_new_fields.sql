-- =====================================================
-- MIGRACIÓN: AGREGAR NUEVOS CAMPOS AVANZADOS
-- Delivery Route Optimizer - Schema Update
-- =====================================================

-- Este archivo permite migrar una base de datos existente
-- para agregar los nuevos campos de funcionalidades avanzadas

BEGIN;

-- =====================================================
-- 1. AGREGAR CAMPOS A TABLA PACKAGES
-- =====================================================

-- Agregar campo para fotos de firma
ALTER TABLE packages 
ADD COLUMN IF NOT EXISTS signature_photo TEXT;

-- Agregar campo para coordenadas de entrega
ALTER TABLE packages 
ADD COLUMN IF NOT EXISTS delivery_coordinates POINT;

-- Agregar campo para duración de entrega
ALTER TABLE packages 
ADD COLUMN IF NOT EXISTS delivery_duration_minutes INTEGER;

-- Agregar constraint para validar duración
ALTER TABLE packages 
ADD CONSTRAINT IF NOT EXISTS valid_delivery_duration 
CHECK (delivery_duration_minutes >= 0 OR delivery_duration_minutes IS NULL);

-- =====================================================
-- 2. AGREGAR CAMPOS A TABLA TOURNEES
-- =====================================================

-- Agregar campo para coordenadas de ruta
ALTER TABLE tournees 
ADD COLUMN IF NOT EXISTS route_coordinates TEXT[];

-- Agregar campo para condiciones de tráfico
ALTER TABLE tournees 
ADD COLUMN IF NOT EXISTS traffic_conditions JSONB;

-- Agregar campo para condiciones meteorológicas
ALTER TABLE tournees 
ADD COLUMN IF NOT EXISTS weather_conditions JSONB;

-- Agregar constraint para validar coordenadas de ruta
ALTER TABLE tournees 
ADD CONSTRAINT IF NOT EXISTS valid_route_coordinates 
CHECK (route_coordinates IS NULL OR array_length(route_coordinates, 1) > 0);

-- =====================================================
-- 3. AGREGAR CAMPOS A TABLA USERS (DRIVERS)
-- =====================================================

-- Agregar campo para token de dispositivo
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS device_token VARCHAR(255);

-- Agregar campo para última ubicación conocida
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS last_location POINT;

-- Agregar campo para hora de inicio de turno
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS shift_start_time TIME;

-- Agregar campo para hora de fin de turno
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS shift_end_time TIME;

-- Agregar constraint para validar horarios de turno
ALTER TABLE users 
ADD CONSTRAINT IF NOT EXISTS valid_shift_times 
CHECK (
    (shift_start_time IS NULL AND shift_end_time IS NULL) OR
    (shift_start_time IS NOT NULL AND shift_end_time IS NOT NULL AND shift_start_time < shift_end_time)
);

-- =====================================================
-- 4. CREAR ÍNDICES PARA LOS NUEVOS CAMPOS
-- =====================================================

-- Índices para packages
CREATE INDEX IF NOT EXISTS idx_packages_delivery_coordinates 
ON packages USING GIST(delivery_coordinates);

CREATE INDEX IF NOT EXISTS idx_packages_delivery_duration 
ON packages(delivery_duration_minutes);

-- Índices para tournees
CREATE INDEX IF NOT EXISTS idx_tournees_traffic_conditions 
ON tournees USING GIN(traffic_conditions);

CREATE INDEX IF NOT EXISTS idx_tournees_weather_conditions 
ON tournees USING GIN(weather_conditions);

-- Índices para users
CREATE INDEX IF NOT EXISTS idx_users_device_token 
ON users(device_token);

CREATE INDEX IF NOT EXISTS idx_users_last_location 
ON users USING GIST(last_location);

CREATE INDEX IF NOT EXISTS idx_users_shift_times 
ON users(shift_start_time, shift_end_time);

-- =====================================================
-- 5. ACTUALIZAR COMENTARIOS DE TABLAS
-- =====================================================

COMMENT ON COLUMN packages.signature_photo IS 'Fotos de firma de entrega para evidencia';
COMMENT ON COLUMN packages.delivery_coordinates IS 'Ubicación exacta de entrega (PostGIS POINT)';
COMMENT ON COLUMN packages.delivery_duration_minutes IS 'Tiempo de entrega en minutos';

COMMENT ON COLUMN tournees.route_coordinates IS 'Ruta completa como array de coordenadas';
COMMENT ON COLUMN tournees.traffic_conditions IS 'Condiciones de tráfico del día (JSONB)';
COMMENT ON COLUMN tournees.weather_conditions IS 'Condiciones meteorológicas (JSONB)';

COMMENT ON COLUMN users.device_token IS 'Token para push notifications';
COMMENT ON COLUMN users.last_location IS 'Última ubicación conocida del chofer (PostGIS POINT)';
COMMENT ON COLUMN users.shift_start_time IS 'Hora de inicio del turno';
COMMENT ON COLUMN users.shift_end_time IS 'Hora de fin del turno';

-- =====================================================
-- 6. VERIFICAR MIGRACIÓN
-- =====================================================

-- Verificar que todos los campos se agregaron correctamente
DO $$
DECLARE
    missing_columns TEXT[] := ARRAY[]::TEXT[];
BEGIN
    -- Verificar campos en packages
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'packages' AND column_name = 'signature_photo') THEN
        missing_columns := array_append(missing_columns, 'packages.signature_photo');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'packages' AND column_name = 'delivery_coordinates') THEN
        missing_columns := array_append(missing_columns, 'packages.delivery_coordinates');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'packages' AND column_name = 'delivery_duration_minutes') THEN
        missing_columns := array_append(missing_columns, 'packages.delivery_duration_minutes');
    END IF;
    
    -- Verificar campos en tournees
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'tournees' AND column_name = 'route_coordinates') THEN
        missing_columns := array_append(missing_columns, 'tournees.route_coordinates');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'tournees' AND column_name = 'traffic_conditions') THEN
        missing_columns := array_append(missing_columns, 'tournees.traffic_conditions');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'tournees' AND column_name = 'weather_conditions') THEN
        missing_columns := array_append(missing_columns, 'tournees.weather_conditions');
    END IF;
    
    -- Verificar campos en users
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'users' AND column_name = 'device_token') THEN
        missing_columns := array_append(missing_columns, 'users.device_token');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'users' AND column_name = 'last_location') THEN
        missing_columns := array_append(missing_columns, 'users.last_location');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'users' AND column_name = 'shift_start_time') THEN
        missing_columns := array_append(missing_columns, 'users.shift_start_time');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'users' AND column_name = 'shift_end_time') THEN
        missing_columns := array_append(missing_columns, 'users.shift_end_time');
    END IF;
    
    -- Reportar resultado
    IF array_length(missing_columns, 1) > 0 THEN
        RAISE NOTICE 'ERROR: Faltan las siguientes columnas: %', array_to_string(missing_columns, ', ');
        RAISE EXCEPTION 'Migración incompleta';
    ELSE
        RAISE NOTICE 'SUCCESS: Todos los campos se agregaron correctamente';
    END IF;
END $$;

-- =====================================================
-- 7. INFORMACIÓN DE MIGRACIÓN
-- =====================================================

RAISE NOTICE '=====================================================';
RAISE NOTICE 'MIGRACIÓN COMPLETADA EXITOSAMENTE';
RAISE NOTICE '=====================================================';
RAISE NOTICE 'Nuevos campos agregados:';
RAISE NOTICE '- packages.signature_photo';
RAISE NOTICE '- packages.delivery_coordinates';
RAISE NOTICE '- packages.delivery_duration_minutes';
RAISE NOTICE '- tournees.route_coordinates';
RAISE NOTICE '- tournees.traffic_conditions';
RAISE NOTICE '- tournees.weather_conditions';
RAISE NOTICE '- users.device_token';
RAISE NOTICE '- users.last_location';
RAISE NOTICE '- users.shift_start_time';
RAISE NOTICE '- users.shift_end_time';
RAISE NOTICE '=====================================================';
RAISE NOTICE 'Próximos pasos:';
RAISE NOTICE '1. Actualizar tu aplicación Rust para usar los nuevos campos';
RAISE NOTICE '2. Implementar lógica para capturar coordenadas GPS';
RAISE NOTICE '3. Configurar push notifications con device_token';
RAISE NOTICE '4. Implementar tracking de condiciones de tráfico y clima';
RAISE NOTICE '=====================================================';

COMMIT;
