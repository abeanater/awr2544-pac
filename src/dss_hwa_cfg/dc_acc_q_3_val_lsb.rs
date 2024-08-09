#[doc = "Register `DC_ACC_Q_3_VAL_LSB` reader"]
pub type R = crate::R<DcAccQ3ValLsbSpec>;
#[doc = "Register `DC_ACC_Q_3_VAL_LSB` writer"]
pub type W = crate::W<DcAccQ3ValLsbSpec>;
#[doc = "Field `dc_acc_q_3_val_lsb` reader - 31:0\\]
This read only register provide the LSB 32 bits value of DC accumulator Q channel value for bcnt=3"]
pub type DcAccQ3ValLsbR = crate::FieldReader<u32>;
#[doc = "Field `dc_acc_q_3_val_lsb` writer - 31:0\\]
This read only register provide the LSB 32 bits value of DC accumulator Q channel value for bcnt=3"]
pub type DcAccQ3ValLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register provide the LSB 32 bits value of DC accumulator Q channel value for bcnt=3"]
    #[inline(always)]
    pub fn dc_acc_q_3_val_lsb(&self) -> DcAccQ3ValLsbR {
        DcAccQ3ValLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register provide the LSB 32 bits value of DC accumulator Q channel value for bcnt=3"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc_q_3_val_lsb(&mut self) -> DcAccQ3ValLsbW<DcAccQ3ValLsbSpec> {
        DcAccQ3ValLsbW::new(self, 0)
    }
}
#[doc = "DC_ACC_Q_3_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_3_val_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_3_val_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAccQ3ValLsbSpec;
impl crate::RegisterSpec for DcAccQ3ValLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc_q_3_val_lsb::R`](R) reader structure"]
impl crate::Readable for DcAccQ3ValLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc_q_3_val_lsb::W`](W) writer structure"]
impl crate::Writable for DcAccQ3ValLsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC_Q_3_VAL_LSB to value 0"]
impl crate::Resettable for DcAccQ3ValLsbSpec {
    const RESET_VALUE: u32 = 0;
}
