#[doc = "Register `INTF_STATS_MAG_ACC_0_LSB` reader"]
pub type R = crate::R<IntfStatsMagAcc0LsbSpec>;
#[doc = "Register `INTF_STATS_MAG_ACC_0_LSB` writer"]
pub type W = crate::W<IntfStatsMagAcc0LsbSpec>;
#[doc = "Field `intf_stats_mag_acc_0_lsb` reader - 31:0\\]
This read only register contains the accumulator value of interference magnitude(LSB 32 bits) for bcnt = 0"]
pub type IntfStatsMagAcc0LsbR = crate::FieldReader<u32>;
#[doc = "Field `intf_stats_mag_acc_0_lsb` writer - 31:0\\]
This read only register contains the accumulator value of interference magnitude(LSB 32 bits) for bcnt = 0"]
pub type IntfStatsMagAcc0LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register contains the accumulator value of interference magnitude(LSB 32 bits) for bcnt = 0"]
    #[inline(always)]
    pub fn intf_stats_mag_acc_0_lsb(&self) -> IntfStatsMagAcc0LsbR {
        IntfStatsMagAcc0LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register contains the accumulator value of interference magnitude(LSB 32 bits) for bcnt = 0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_acc_0_lsb(&mut self) -> IntfStatsMagAcc0LsbW<IntfStatsMagAcc0LsbSpec> {
        IntfStatsMagAcc0LsbW::new(self, 0)
    }
}
#[doc = "INTF_STATS_MAG_ACC_0_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_0_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_0_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsMagAcc0LsbSpec;
impl crate::RegisterSpec for IntfStatsMagAcc0LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_mag_acc_0_lsb::R`](R) reader structure"]
impl crate::Readable for IntfStatsMagAcc0LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_mag_acc_0_lsb::W`](W) writer structure"]
impl crate::Writable for IntfStatsMagAcc0LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_MAG_ACC_0_LSB to value 0"]
impl crate::Resettable for IntfStatsMagAcc0LsbSpec {
    const RESET_VALUE: u32 = 0;
}
