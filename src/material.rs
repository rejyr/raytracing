use std::sync::Arc;

use crate::{
    color::Color,
    helper::random_f64,
    hittable::HitRecord,
    pdf::{CosinePDF, PDF, SpherePDF},
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::{Point3, Vec3},
};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub surface: ScatterSurface,
}

#[derive(Clone)]
pub enum ScatterSurface {
    Diffuse { pdf: Arc<dyn PDF> },
    Specular { skip_pdf_ray: Ray },
}

#[macro_export]
macro_rules! material {
    (Lambertian( $tex:expr )) => {
        std::sync::Arc::new($crate::material::Lambertian::new($tex.clone()))
    };
    (Lambertian( color: $color:expr )) => {
        std::sync::Arc::new($crate::material::Lambertian::new_from_color(&($color)))
    };
    (Metal( $albedo:expr, $fuzz:expr )) => {
        std::sync::Arc::new($crate::material::Metal::new(&($albedo), $fuzz as f64))
    };
    (Dielectric( $ri:expr )) => {
        std::sync::Arc::new($crate::material::Dielectric::new($ri as f64))
    };
    (DiffuseLight( $tex:expr )) => {
        std::sync::Arc::new($crate::material::DiffuseLight::new($tex.clone()))
    };
    (DiffuseLight( color: $color:expr )) => {
        std::sync::Arc::new($crate::material::DiffuseLight::new_from_color(&($color)))
    };
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf = Arc::new(CosinePDF::new(&rec.normal));

        Some(ScatterRecord {
            attenuation,
            surface: ScatterSurface::Diffuse { pdf },
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(&scattered.direction().unit_vector());
        (cos_theta / std::f64::consts::PI).max(0.0)
    }
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_from_color(albedo: &Color) -> Self {
        Self::new(Arc::new(SolidColor::new(albedo)))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction().reflect(&rec.normal);
        let reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());

        let attenuation = self.albedo;
        let skip_pdf_ray = Ray::new_with_time(rec.p, reflected, r_in.time());

        Some(ScatterRecord {
            attenuation,
            surface: ScatterSurface::Specular { skip_pdf_ray },
        })
    }
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_f64() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, ri)
        };

        let skip_pdf_ray = Ray::new_with_time(rec.p, direction, r_in.time());
        Some(ScatterRecord {
            attenuation,
            surface: ScatterSurface::Specular { skip_pdf_ray },
        })
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        self.tex.value(u, v, p)
    }
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_from_color(emit: &Color) -> Self {
        Self::new(Arc::new(SolidColor::new(emit)))
    }
}

#[derive(Clone)]
pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf = Arc::new(SpherePDF);
        Some(ScatterRecord {
            attenuation,
            surface: ScatterSurface::Diffuse { pdf },
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_from_color(albedo: &Color) -> Self {
        Self::new(Arc::new(SolidColor::new(albedo)))
    }
}
