#[doc = "Register `INTF_STATS_MAGDIFF_ACC_0_LSB` reader"]
pub type R = crate::R<IntfStatsMagdiffAcc0LsbSpec>;
#[doc = "Register `INTF_STATS_MAGDIFF_ACC_0_LSB` writer"]
pub type W = crate::W<IntfStatsMagdiffAcc0LsbSpec>;
#[doc = "Field `intf_stats_magdiff_acc_0_lsb` reader - 31:0\\]
This read only register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 0"]
pub type IntfStatsMagdiffAcc0LsbR = crate::FieldReader<u32>;
#[doc = "Field `intf_stats_magdiff_acc_0_lsb` writer - 31:0\\]
This read only register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 0"]
pub type IntfStatsMagdiffAcc0LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 0"]
    #[inline(always)]
    pub fn intf_stats_magdiff_acc_0_lsb(&self) -> IntfStatsMagdiffAcc0LsbR {
        IntfStatsMagdiffAcc0LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This read only register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_magdiff_acc_0_lsb(
        &mut self,
    ) -> IntfStatsMagdiffAcc0LsbW<IntfStatsMagdiffAcc0LsbSpec> {
        IntfStatsMagdiffAcc0LsbW::new(self, 0)
    }
}
#[doc = "INTF_STATS_MAGDIFF_ACC_0_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_0_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_0_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsMagdiffAcc0LsbSpec;
impl crate::RegisterSpec for IntfStatsMagdiffAcc0LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_magdiff_acc_0_lsb::R`](R) reader structure"]
impl crate::Readable for IntfStatsMagdiffAcc0LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_magdiff_acc_0_lsb::W`](W) writer structure"]
impl crate::Writable for IntfStatsMagdiffAcc0LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_MAGDIFF_ACC_0_LSB to value 0"]
impl crate::Resettable for IntfStatsMagdiffAcc0LsbSpec {
    const RESET_VALUE: u32 = 0;
}
