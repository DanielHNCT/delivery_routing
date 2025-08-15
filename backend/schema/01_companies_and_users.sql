-- =====================================================
-- DELIVERY ROUTE OPTIMIZER - SCHEMA COMPLETO
-- Plataforma SaaS Multi-Tenant para Optimización de Rutas
-- =====================================================

-- Habilitar extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";

-- =====================================================
-- NIVEL 1: COMPANIES (Contenedor principal)
-- =====================================================

CREATE TABLE companies (
    company_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    subscription_plan VARCHAR(50) NOT NULL DEFAULT 'basic',
    subscription_status VARCHAR(20) NOT NULL DEFAULT 'active',
    max_drivers INTEGER NOT NULL DEFAULT 10,
    max_vehicles INTEGER NOT NULL DEFAULT 5,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Índices para companies
CREATE INDEX idx_companies_subscription_status ON companies(subscription_status);
CREATE INDEX idx_companies_deleted_at ON companies(deleted_at);

-- =====================================================
-- NIVEL 2A: USERS (Dentro de cada company)
-- =====================================================

CREATE TYPE user_type AS ENUM ('admin', 'driver');
CREATE TYPE user_status AS ENUM ('active', 'inactive', 'suspended');

CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    user_type user_type NOT NULL,
    user_status user_status NOT NULL DEFAULT 'active',
    
    -- Campos comunes
    username VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(20),
    
    -- Campos específicos para drivers
    tournee_number VARCHAR(20),
    driver_license VARCHAR(50),
    hire_date DATE,
    device_token VARCHAR(255), -- Para push notifications
    last_location POINT, -- Última ubicación conocida (PostGIS)
    shift_start_time TIME, -- Hora de inicio del turno
    shift_end_time TIME, -- Hora de fin del turno
    
    -- Campos específicos para admins
    permissions JSONB DEFAULT '{}',
    
    -- Metadatos
    last_login TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT unique_username_per_company UNIQUE (company_id, username),
    CONSTRAINT unique_email_per_company UNIQUE (company_id, email),
    CONSTRAINT max_admins_per_company CHECK (
        (user_type = 'admin' AND (
            SELECT COUNT(*) FROM users u2 
            WHERE u2.company_id = company_id 
            AND u2.user_type = 'admin' 
            AND u2.deleted_at IS NULL
        ) <= 2
        ) OR user_type = 'driver'
    ),
    CONSTRAINT valid_shift_times CHECK (
        (shift_start_time IS NULL AND shift_end_time IS NULL) OR
        (shift_start_time IS NOT NULL AND shift_end_time IS NOT NULL AND shift_start_time < shift_end_time)
    )
);

-- Índices para users
CREATE INDEX idx_users_company_id ON users(company_id);
CREATE INDEX idx_users_user_type ON users(user_type);
CREATE INDEX idx_users_user_status ON users(user_status);
CREATE INDEX idx_users_tournee_number ON users(tournee_number);
CREATE INDEX idx_users_deleted_at ON users(deleted_at);
CREATE INDEX idx_users_company_type ON users(company_id, user_type);
CREATE INDEX idx_users_device_token ON users(device_token);
CREATE INDEX idx_users_last_location ON users USING GIST(last_location);
CREATE INDEX idx_users_shift_times ON users(shift_start_time, shift_end_time);

-- =====================================================
-- NIVEL 2B: VEHICLES (Dentro de cada company)
-- =====================================================

CREATE TYPE vehicle_status AS ENUM ('active', 'maintenance', 'out_of_service', 'retired');

CREATE TABLE vehicles (
    vehicle_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES companies(company_id) ON DELETE CASCADE,
    license_plate VARCHAR(20) NOT NULL,
    brand VARCHAR(100) NOT NULL,
    model VARCHAR(100) NOT NULL,
    year INTEGER,
    color VARCHAR(50),
    
    -- Estado operativo
    vehicle_status vehicle_status NOT NULL DEFAULT 'active',
    current_mileage DECIMAL(10,2) NOT NULL DEFAULT 0,
    fuel_type VARCHAR(20) NOT NULL DEFAULT 'diesel',
    fuel_capacity DECIMAL(5,2), -- en litros
    weekly_fuel_allocation DECIMAL(5,2), -- medio tanque semanal
    
    -- Métricas de daños
    total_damage_cost DECIMAL(10,2) NOT NULL DEFAULT 0,
    damage_incidents_count INTEGER NOT NULL DEFAULT 0,
    
    -- Información técnica
    vin VARCHAR(17),
    engine_size VARCHAR(20),
    transmission VARCHAR(20),
    
    -- Metadatos
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT unique_license_plate_per_company UNIQUE (company_id, license_plate),
    CONSTRAINT positive_mileage CHECK (current_mileage >= 0),
    CONSTRAINT positive_fuel_allocation CHECK (weekly_fuel_allocation > 0)
);

-- Índices para vehicles
CREATE INDEX idx_vehicles_company_id ON vehicles(company_id);
CREATE INDEX idx_vehicles_license_plate ON vehicles(license_plate);
CREATE INDEX idx_vehicles_status ON vehicles(vehicle_status);
CREATE INDEX idx_vehicles_deleted_at ON vehicles(deleted_at);

-- =====================================================
-- NIVEL 2C: API_INTEGRATIONS (Integraciones con APIs externas)
-- =====================================================

CREATE TYPE sync_status AS ENUM ('active', 'error', 'disabled', 'syncing');

CREATE TABLE api_integrations (
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

-- Índices para api_integrations
CREATE INDEX idx_api_integrations_company_id ON api_integrations(company_id);
CREATE INDEX idx_api_integrations_provider_name ON api_integrations(provider_name);
CREATE INDEX idx_api_integrations_sync_status ON api_integrations(sync_status);
CREATE INDEX idx_api_integrations_last_sync ON api_integrations(last_sync_date);
CREATE INDEX idx_api_integrations_company_provider ON api_integrations(company_id, provider_name);
CREATE INDEX idx_api_integrations_status_provider ON api_integrations(sync_status, provider_name);
