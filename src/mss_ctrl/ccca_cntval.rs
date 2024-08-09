#[doc = "Register `CCCA_CNTVAL` reader"]
pub type R = crate::R<CccaCntvalSpec>;
#[doc = "Register `CCCA_CNTVAL` writer"]
pub type W = crate::W<CccaCntvalSpec>;
#[doc = "Field `ccca_cfg` reader - 31:0\\]
count1_val_out Real time value of counter1"]
pub type CccaCfgR = crate::FieldReader<u32>;
#[doc = "Field `ccca_cfg` writer - 31:0\\]
count1_val_out Real time value of counter1"]
pub type CccaCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
count1_val_out Real time value of counter1"]
    #[inline(always)]
    pub fn ccca_cfg(&self) -> CccaCfgR {
        CccaCfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
count1_val_out Real time value of counter1"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_cfg(&mut self) -> CccaCfgW<CccaCntvalSpec> {
        CccaCfgW::new(self, 0)
    }
}
#[doc = "CCCA_CNTVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cntval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cntval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccaCntvalSpec;
impl crate::RegisterSpec for CccaCntvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccca_cntval::R`](R) reader structure"]
impl crate::Readable for CccaCntvalSpec {}
#[doc = "`write(|w| ..)` method takes [`ccca_cntval::W`](W) writer structure"]
impl crate::Writable for CccaCntvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCA_CNTVAL to value 0"]
impl crate::Resettable for CccaCntvalSpec {
    const RESET_VALUE: u32 = 0;
}
