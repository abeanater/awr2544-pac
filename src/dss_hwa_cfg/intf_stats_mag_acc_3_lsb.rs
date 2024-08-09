#[doc = "Register `INTF_STATS_MAG_ACC_3_LSB` reader"]
pub type R = crate::R<IntfStatsMagAcc3LsbSpec>;
#[doc = "Register `INTF_STATS_MAG_ACC_3_LSB` writer"]
pub type W = crate::W<IntfStatsMagAcc3LsbSpec>;
#[doc = "Field `intf_stats_mag_acc_3_lsb` reader - 31:0\\]
This read only register contains the accumulator value of the interference magnitude( for LSB 32 bits) for bcnt = 3"]
pub type IntfStatsMagAcc3LsbR = crate::FieldReader<u32>;
#[doc = "Field `intf_stats_mag_acc_3_lsb` writer - 31:0\\]
This read only register contains the accumulator value of the interference magnitude( for LSB 32 bits) for bcnt = 3"]
pub type IntfStatsMagAcc3LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register contains the accumulator value of the interference magnitude( for LSB 32 bits) for bcnt = 3"]
    #[inline(always)]
    pub fn intf_stats_mag_acc_3_lsb(&self) -> IntfStatsMagAcc3LsbR {
        IntfStatsMagAcc3LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register contains the accumulator value of the interference magnitude( for LSB 32 bits) for bcnt = 3"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_acc_3_lsb(&mut self) -> IntfStatsMagAcc3LsbW<IntfStatsMagAcc3LsbSpec> {
        IntfStatsMagAcc3LsbW::new(self, 0)
    }
}
#[doc = "INTF_STATS_MAG_ACC_3_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_3_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_3_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsMagAcc3LsbSpec;
impl crate::RegisterSpec for IntfStatsMagAcc3LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_mag_acc_3_lsb::R`](R) reader structure"]
impl crate::Readable for IntfStatsMagAcc3LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_mag_acc_3_lsb::W`](W) writer structure"]
impl crate::Writable for IntfStatsMagAcc3LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_MAG_ACC_3_LSB to value 0"]
impl crate::Resettable for IntfStatsMagAcc3LsbSpec {
    const RESET_VALUE: u32 = 0;
}
