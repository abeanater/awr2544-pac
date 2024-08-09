#[doc = "Register `MSS_DMM_ACCESS_MODE` reader"]
pub type R = crate::R<MssDmmAccessModeSpec>;
#[doc = "Register `MSS_DMM_ACCESS_MODE` writer"]
pub type W = crate::W<MssDmmAccessModeSpec>;
#[doc = "Field `dmma_sel` reader - 0:0\\]
writing 1'b0 : ensures all the accesses from DMMA are user-mode writing 1'b1 : ensures all the accesses from DMMA are privilege mode"]
pub type DmmaSelR = crate::BitReader;
#[doc = "Field `dmma_sel` writer - 0:0\\]
writing 1'b0 : ensures all the accesses from DMMA are user-mode writing 1'b1 : ensures all the accesses from DMMA are privilege mode"]
pub type DmmaSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmmb_sel` reader - 4:4\\]
writing 1'b0 : ensures all the accesses from DMMB are user-mode writing 1'b1 : ensures all the accesses from DMMB are privilege mode"]
pub type DmmbSelR = crate::BitReader;
#[doc = "Field `dmmb_sel` writer - 4:4\\]
writing 1'b0 : ensures all the accesses from DMMB are user-mode writing 1'b1 : ensures all the accesses from DMMB are privilege mode"]
pub type DmmbSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b0 : ensures all the accesses from DMMA are user-mode writing 1'b1 : ensures all the accesses from DMMA are privilege mode"]
    #[inline(always)]
    pub fn dmma_sel(&self) -> DmmaSelR {
        DmmaSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
writing 1'b0 : ensures all the accesses from DMMB are user-mode writing 1'b1 : ensures all the accesses from DMMB are privilege mode"]
    #[inline(always)]
    pub fn dmmb_sel(&self) -> DmmbSelR {
        DmmbSelR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b0 : ensures all the accesses from DMMA are user-mode writing 1'b1 : ensures all the accesses from DMMA are privilege mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmma_sel(&mut self) -> DmmaSelW<MssDmmAccessModeSpec> {
        DmmaSelW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
writing 1'b0 : ensures all the accesses from DMMB are user-mode writing 1'b1 : ensures all the accesses from DMMB are privilege mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmmb_sel(&mut self) -> DmmbSelW<MssDmmAccessModeSpec> {
        DmmbSelW::new(self, 4)
    }
}
#[doc = "MSS_DMM_ACCESS_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_access_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_access_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmAccessModeSpec;
impl crate::RegisterSpec for MssDmmAccessModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_access_mode::R`](R) reader structure"]
impl crate::Readable for MssDmmAccessModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_access_mode::W`](W) writer structure"]
impl crate::Writable for MssDmmAccessModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_ACCESS_MODE to value 0"]
impl crate::Resettable for MssDmmAccessModeSpec {
    const RESET_VALUE: u32 = 0;
}
