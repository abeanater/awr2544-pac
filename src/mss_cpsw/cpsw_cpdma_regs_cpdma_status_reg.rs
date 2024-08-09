#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_STATUS_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaStatusRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_STATUS_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaStatusRegSpec>;
#[doc = "Field `CPDMA_THOST_ERROR` reader - 10:8\\]
CPDMA THost Error Channel Number"]
pub type CpdmaThostErrorR = crate::FieldReader;
#[doc = "Field `CPDMA_THOST_ERROR` writer - 10:8\\]
CPDMA THost Error Channel Number"]
pub type CpdmaThostErrorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPDMA_THOST_HOST` reader - 15:12\\]
CPDMA THost Host Error Code"]
pub type CpdmaThostHostR = crate::FieldReader;
#[doc = "Field `CPDMA_THOST_HOST` writer - 15:12\\]
CPDMA THost Host Error Code"]
pub type CpdmaThostHostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPDMA_FHOST_ERROR` reader - 18:16\\]
CPDMA FHost Error Channel Number"]
pub type CpdmaFhostErrorR = crate::FieldReader;
#[doc = "Field `CPDMA_FHOST_ERROR` writer - 18:16\\]
CPDMA FHost Error Channel Number"]
pub type CpdmaFhostErrorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPDMA_FHOST_HOST` reader - 23:20\\]
CPDMA FHost Host Error Code"]
pub type CpdmaFhostHostR = crate::FieldReader;
#[doc = "Field `CPDMA_FHOST_HOST` writer - 23:20\\]
CPDMA FHost Host Error Code"]
pub type CpdmaFhostHostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPDMA_FHOST_HOST` reader - 31:31\\]
CPDMA FHost Host Error Code"]
pub type CpdmaFhostHostR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_HOST` writer - 31:31\\]
CPDMA FHost Host Error Code"]
pub type CpdmaFhostHostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:10 - 10:8\\]
CPDMA THost Error Channel Number"]
    #[inline(always)]
    pub fn cpdma_thost_error(&self) -> CpdmaThostErrorR {
        CpdmaThostErrorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
CPDMA THost Host Error Code"]
    #[inline(always)]
    pub fn cpdma_thost_host(&self) -> CpdmaThostHostR {
        CpdmaThostHostR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
CPDMA FHost Error Channel Number"]
    #[inline(always)]
    pub fn cpdma_fhost_error(&self) -> CpdmaFhostErrorR {
        CpdmaFhostErrorR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
CPDMA FHost Host Error Code"]
    #[inline(always)]
    pub fn cpdma_fhost_host(&self) -> CpdmaFhostHostR {
        CpdmaFhostHostR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
CPDMA FHost Host Error Code"]
    #[inline(always)]
    pub fn cpdma_fhost_host(&self) -> CpdmaFhostHostR {
        CpdmaFhostHostR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - 10:8\\]
CPDMA THost Error Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_error(&mut self) -> CpdmaThostErrorW<CpswCpdmaRegsCpdmaStatusRegSpec> {
        CpdmaThostErrorW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
CPDMA THost Host Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_host(&mut self) -> CpdmaThostHostW<CpswCpdmaRegsCpdmaStatusRegSpec> {
        CpdmaThostHostW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
CPDMA FHost Error Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_error(&mut self) -> CpdmaFhostErrorW<CpswCpdmaRegsCpdmaStatusRegSpec> {
        CpdmaFhostErrorW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
CPDMA FHost Host Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_host(&mut self) -> CpdmaFhostHostW<CpswCpdmaRegsCpdmaStatusRegSpec> {
        CpdmaFhostHostW::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
CPDMA FHost Host Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_host(&mut self) -> CpdmaFhostHostW<CpswCpdmaRegsCpdmaStatusRegSpec> {
        CpdmaFhostHostW::new(self, 31)
    }
}
#[doc = "CPDMA Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaStatusRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_STATUS_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
