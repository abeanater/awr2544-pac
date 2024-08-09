#[doc = "Register `CCCB_CNTVAL` reader"]
pub type R = crate::R<CccbCntvalSpec>;
#[doc = "Register `CCCB_CNTVAL` writer"]
pub type W = crate::W<CccbCntvalSpec>;
#[doc = "Field `cccb_cfg` reader - 31:0\\]
count1_val_out Real time value of counter1"]
pub type CccbCfgR = crate::FieldReader<u32>;
#[doc = "Field `cccb_cfg` writer - 31:0\\]
count1_val_out Real time value of counter1"]
pub type CccbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
count1_val_out Real time value of counter1"]
    #[inline(always)]
    pub fn cccb_cfg(&self) -> CccbCfgR {
        CccbCfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
count1_val_out Real time value of counter1"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_cfg(&mut self) -> CccbCfgW<CccbCntvalSpec> {
        CccbCfgW::new(self, 0)
    }
}
#[doc = "CCCB_CNTVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cntval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cntval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccbCntvalSpec;
impl crate::RegisterSpec for CccbCntvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccb_cntval::R`](R) reader structure"]
impl crate::Readable for CccbCntvalSpec {}
#[doc = "`write(|w| ..)` method takes [`cccb_cntval::W`](W) writer structure"]
impl crate::Writable for CccbCntvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCB_CNTVAL to value 0"]
impl crate::Resettable for CccbCntvalSpec {
    const RESET_VALUE: u32 = 0;
}
