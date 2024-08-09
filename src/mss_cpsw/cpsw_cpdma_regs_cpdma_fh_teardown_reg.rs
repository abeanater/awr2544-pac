#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_TEARDOWN_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaFhTeardownRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_TEARDOWN_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaFhTeardownRegSpec>;
#[doc = "Field `CPDMA_FHOST_TEARDOWN_1` reader - 2:0\\]
CPDMA FHost Teardown Channel"]
pub type CpdmaFhostTeardown1R = crate::FieldReader;
#[doc = "Field `CPDMA_FHOST_TEARDOWN_1` writer - 2:0\\]
CPDMA FHost Teardown Channel"]
pub type CpdmaFhostTeardown1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPDMA_FHOST_TEARDOWN` reader - 31:31\\]
CPDMA FHost Teardown Ready"]
pub type CpdmaFhostTeardownR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_TEARDOWN` writer - 31:31\\]
CPDMA FHost Teardown Ready"]
pub type CpdmaFhostTeardownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
CPDMA FHost Teardown Channel"]
    #[inline(always)]
    pub fn cpdma_fhost_teardown_1(&self) -> CpdmaFhostTeardown1R {
        CpdmaFhostTeardown1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
CPDMA FHost Teardown Ready"]
    #[inline(always)]
    pub fn cpdma_fhost_teardown(&self) -> CpdmaFhostTeardownR {
        CpdmaFhostTeardownR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
CPDMA FHost Teardown Channel"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_teardown_1(
        &mut self,
    ) -> CpdmaFhostTeardown1W<CpswCpdmaRegsCpdmaFhTeardownRegSpec> {
        CpdmaFhostTeardown1W::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
CPDMA FHost Teardown Ready"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_teardown(
        &mut self,
    ) -> CpdmaFhostTeardownW<CpswCpdmaRegsCpdmaFhTeardownRegSpec> {
        CpdmaFhostTeardownW::new(self, 31)
    }
}
#[doc = "CPDMA FHost Teardown Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_teardown_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_teardown_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaFhTeardownRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaFhTeardownRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_fh_teardown_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaFhTeardownRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_fh_teardown_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaFhTeardownRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_FH_TEARDOWN_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaFhTeardownRegSpec {
    const RESET_VALUE: u32 = 0;
}
