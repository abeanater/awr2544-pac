#[doc = "Register `INTF_STATS_MAG_ACC_2_MSB` reader"]
pub type R = crate::R<IntfStatsMagAcc2MsbSpec>;
#[doc = "Register `INTF_STATS_MAG_ACC_2_MSB` writer"]
pub type W = crate::W<IntfStatsMagAcc2MsbSpec>;
#[doc = "Field `intf_stats_mag_acc_2_msb` reader - 3:0\\]
This read only register contains the accumulator value of interference magnitude (MSB 4 bits) for bcnt = 2"]
pub type IntfStatsMagAcc2MsbR = crate::FieldReader;
#[doc = "Field `intf_stats_mag_acc_2_msb` writer - 3:0\\]
This read only register contains the accumulator value of interference magnitude (MSB 4 bits) for bcnt = 2"]
pub type IntfStatsMagAcc2MsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register contains the accumulator value of interference magnitude (MSB 4 bits) for bcnt = 2"]
    #[inline(always)]
    pub fn intf_stats_mag_acc_2_msb(&self) -> IntfStatsMagAcc2MsbR {
        IntfStatsMagAcc2MsbR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register contains the accumulator value of interference magnitude (MSB 4 bits) for bcnt = 2"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_acc_2_msb(&mut self) -> IntfStatsMagAcc2MsbW<IntfStatsMagAcc2MsbSpec> {
        IntfStatsMagAcc2MsbW::new(self, 0)
    }
}
#[doc = "INTF_STATS_MAG_ACC_2_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_2_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_2_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsMagAcc2MsbSpec;
impl crate::RegisterSpec for IntfStatsMagAcc2MsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_mag_acc_2_msb::R`](R) reader structure"]
impl crate::Readable for IntfStatsMagAcc2MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_mag_acc_2_msb::W`](W) writer structure"]
impl crate::Writable for IntfStatsMagAcc2MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_MAG_ACC_2_MSB to value 0"]
impl crate::Resettable for IntfStatsMagAcc2MsbSpec {
    const RESET_VALUE: u32 = 0;
}
