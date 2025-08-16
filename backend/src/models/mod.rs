//! Modelos del sistema
//! 
//! Este módulo contiene todos los modelos de datos que mapean exactamente
//! al schema PostgreSQL con las convenciones estándar.

pub mod auth;
pub mod company;
pub mod user;
pub mod vehicle;
pub mod tournee;
pub mod package;
pub mod analytics;

// Re-export specific types to avoid conflicts
pub use auth::{LoginRequest};
pub use company::{Company, CompanyResponse, CreateCompanyRequest, UpdateCompanyRequest, CompanyListResponse, CompanyFilters};
pub use user::{User, UserResponse, CreateUserRequest, UpdateUserRequest, UserListResponse, UserFilters, UserLoginRequest, UserLoginResponse};
pub use vehicle::{Vehicle, VehicleResponse, CreateVehicleRequest, UpdateVehicleRequest, VehicleListResponse, VehicleFilters};
pub use tournee::{Tournee, TourneeResponse, CreateTourneeRequest, UpdateTourneeRequest, TourneeListResponse, TourneeFilters};
pub use package::{Package, PackageResponse, CreatePackageRequest, UpdatePackageRequest, PackageListResponse, PackageFilters};
pub use analytics::{PerformanceAnalytics, DashboardSummary, CreateAnalyticsRequest, UpdateAnalyticsRequest};
