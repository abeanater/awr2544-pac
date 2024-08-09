#[doc = "Register `INTF_STATS_MAGDIFF_ACC_1_MSB` reader"]
pub type R = crate::R<IntfStatsMagdiffAcc1MsbSpec>;
#[doc = "Register `INTF_STATS_MAGDIFF_ACC_1_MSB` writer"]
pub type W = crate::W<IntfStatsMagdiffAcc1MsbSpec>;
#[doc = "Field `intf_stats_magdiff_acc_1_msb` reader - 3:0\\]
This read only register contains the accumulator value of the interference magnitude difference (MSB 4 bits) for bcnt = 1"]
pub type IntfStatsMagdiffAcc1MsbR = crate::FieldReader;
#[doc = "Field `intf_stats_magdiff_acc_1_msb` writer - 3:0\\]
This read only register contains the accumulator value of the interference magnitude difference (MSB 4 bits) for bcnt = 1"]
pub type IntfStatsMagdiffAcc1MsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register contains the accumulator value of the interference magnitude difference (MSB 4 bits) for bcnt = 1"]
    #[inline(always)]
    pub fn intf_stats_magdiff_acc_1_msb(&self) -> IntfStatsMagdiffAcc1MsbR {
        IntfStatsMagdiffAcc1MsbR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register contains the accumulator value of the interference magnitude difference (MSB 4 bits) for bcnt = 1"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_magdiff_acc_1_msb(
        &mut self,
    ) -> IntfStatsMagdiffAcc1MsbW<IntfStatsMagdiffAcc1MsbSpec> {
        IntfStatsMagdiffAcc1MsbW::new(self, 0)
    }
}
#[doc = "INTF_STATS_MAGDIFF_ACC_1_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_1_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_1_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsMagdiffAcc1MsbSpec;
impl crate::RegisterSpec for IntfStatsMagdiffAcc1MsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_magdiff_acc_1_msb::R`](R) reader structure"]
impl crate::Readable for IntfStatsMagdiffAcc1MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_magdiff_acc_1_msb::W`](W) writer structure"]
impl crate::Writable for IntfStatsMagdiffAcc1MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_MAGDIFF_ACC_1_MSB to value 0"]
impl crate::Resettable for IntfStatsMagdiffAcc1MsbSpec {
    const RESET_VALUE: u32 = 0;
}
