#[doc = "Register `DC_ACC_Q_4_VAL_MSB` reader"]
pub type R = crate::R<DcAccQ4ValMsbSpec>;
#[doc = "Register `DC_ACC_Q_4_VAL_MSB` writer"]
pub type W = crate::W<DcAccQ4ValMsbSpec>;
#[doc = "Field `dc_acc_q_4_val_msb` reader - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator Q channel value for bcnt=4"]
pub type DcAccQ4ValMsbR = crate::FieldReader;
#[doc = "Field `dc_acc_q_4_val_msb` writer - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator Q channel value for bcnt=4"]
pub type DcAccQ4ValMsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator Q channel value for bcnt=4"]
    #[inline(always)]
    pub fn dc_acc_q_4_val_msb(&self) -> DcAccQ4ValMsbR {
        DcAccQ4ValMsbR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register provide the MSB 4 bits value of DC accumulator Q channel value for bcnt=4"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc_q_4_val_msb(&mut self) -> DcAccQ4ValMsbW<DcAccQ4ValMsbSpec> {
        DcAccQ4ValMsbW::new(self, 0)
    }
}
#[doc = "DC_ACC_Q_4_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_4_val_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_4_val_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAccQ4ValMsbSpec;
impl crate::RegisterSpec for DcAccQ4ValMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc_q_4_val_msb::R`](R) reader structure"]
impl crate::Readable for DcAccQ4ValMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc_q_4_val_msb::W`](W) writer structure"]
impl crate::Writable for DcAccQ4ValMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC_Q_4_VAL_MSB to value 0"]
impl crate::Resettable for DcAccQ4ValMsbSpec {
    const RESET_VALUE: u32 = 0;
}
