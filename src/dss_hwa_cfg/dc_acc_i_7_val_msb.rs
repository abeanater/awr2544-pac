#[doc = "Register `DC_ACC_I_7_VAL_MSB` reader"]
pub type R = crate::R<DcAccI7ValMsbSpec>;
#[doc = "Register `DC_ACC_I_7_VAL_MSB` writer"]
pub type W = crate::W<DcAccI7ValMsbSpec>;
#[doc = "Field `dc_acc_i_7_val_msb` reader - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=7"]
pub type DcAccI7ValMsbR = crate::FieldReader;
#[doc = "Field `dc_acc_i_7_val_msb` writer - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=7"]
pub type DcAccI7ValMsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=7"]
    #[inline(always)]
    pub fn dc_acc_i_7_val_msb(&self) -> DcAccI7ValMsbR {
        DcAccI7ValMsbR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator I channel value for bcnt=7"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc_i_7_val_msb(&mut self) -> DcAccI7ValMsbW<DcAccI7ValMsbSpec> {
        DcAccI7ValMsbW::new(self, 0)
    }
}
#[doc = "DC_ACC_I_7_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_7_val_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_7_val_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAccI7ValMsbSpec;
impl crate::RegisterSpec for DcAccI7ValMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc_i_7_val_msb::R`](R) reader structure"]
impl crate::Readable for DcAccI7ValMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc_i_7_val_msb::W`](W) writer structure"]
impl crate::Writable for DcAccI7ValMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC_I_7_VAL_MSB to value 0"]
impl crate::Resettable for DcAccI7ValMsbSpec {
    const RESET_VALUE: u32 = 0;
}
