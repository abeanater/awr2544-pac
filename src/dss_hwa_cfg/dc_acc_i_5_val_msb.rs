#[doc = "Register `DC_ACC_I_5_VAL_MSB` reader"]
pub type R = crate::R<DcAccI5ValMsbSpec>;
#[doc = "Register `DC_ACC_I_5_VAL_MSB` writer"]
pub type W = crate::W<DcAccI5ValMsbSpec>;
#[doc = "Field `dc_acc_i_5_val_msb` reader - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=5"]
pub type DcAccI5ValMsbR = crate::FieldReader;
#[doc = "Field `dc_acc_i_5_val_msb` writer - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=5"]
pub type DcAccI5ValMsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=5"]
    #[inline(always)]
    pub fn dc_acc_i_5_val_msb(&self) -> DcAccI5ValMsbR {
        DcAccI5ValMsbR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=5"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc_i_5_val_msb(&mut self) -> DcAccI5ValMsbW<DcAccI5ValMsbSpec> {
        DcAccI5ValMsbW::new(self, 0)
    }
}
#[doc = "DC_ACC_I_5_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_5_val_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_5_val_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAccI5ValMsbSpec;
impl crate::RegisterSpec for DcAccI5ValMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc_i_5_val_msb::R`](R) reader structure"]
impl crate::Readable for DcAccI5ValMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc_i_5_val_msb::W`](W) writer structure"]
impl crate::Writable for DcAccI5ValMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC_I_5_VAL_MSB to value 0"]
impl crate::Resettable for DcAccI5ValMsbSpec {
    const RESET_VALUE: u32 = 0;
}
