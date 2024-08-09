#[doc = "Register `INTF_STATS_MAG_ACC_2_LSB` reader"]
pub type R = crate::R<IntfStatsMagAcc2LsbSpec>;
#[doc = "Register `INTF_STATS_MAG_ACC_2_LSB` writer"]
pub type W = crate::W<IntfStatsMagAcc2LsbSpec>;
#[doc = "Field `intf_stats_mag_acc_2_lsb` reader - 31:0\\]
This read only contains the accumulator value of interference magnitude (LSB 32 bits) for bcnt = 2"]
pub type IntfStatsMagAcc2LsbR = crate::FieldReader<u32>;
#[doc = "Field `intf_stats_mag_acc_2_lsb` writer - 31:0\\]
This read only contains the accumulator value of interference magnitude (LSB 32 bits) for bcnt = 2"]
pub type IntfStatsMagAcc2LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This read only contains the accumulator value of interference magnitude (LSB 32 bits) for bcnt = 2"]
    #[inline(always)]
    pub fn intf_stats_mag_acc_2_lsb(&self) -> IntfStatsMagAcc2LsbR {
        IntfStatsMagAcc2LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This read only contains the accumulator value of interference magnitude (LSB 32 bits) for bcnt = 2"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_acc_2_lsb(&mut self) -> IntfStatsMagAcc2LsbW<IntfStatsMagAcc2LsbSpec> {
        IntfStatsMagAcc2LsbW::new(self, 0)
    }
}
#[doc = "INTF_STATS_MAG_ACC_2_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_2_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_2_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsMagAcc2LsbSpec;
impl crate::RegisterSpec for IntfStatsMagAcc2LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_mag_acc_2_lsb::R`](R) reader structure"]
impl crate::Readable for IntfStatsMagAcc2LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_mag_acc_2_lsb::W`](W) writer structure"]
impl crate::Writable for IntfStatsMagAcc2LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_MAG_ACC_2_LSB to value 0"]
impl crate::Resettable for IntfStatsMagAcc2LsbSpec {
    const RESET_VALUE: u32 = 0;
}
