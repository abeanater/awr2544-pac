#[doc = "Register `INTF_STATS_MAG_ACC_0_MSB` reader"]
pub type R = crate::R<IntfStatsMagAcc0MsbSpec>;
#[doc = "Register `INTF_STATS_MAG_ACC_0_MSB` writer"]
pub type W = crate::W<IntfStatsMagAcc0MsbSpec>;
#[doc = "Field `intf_stats_mag_acc_0_msb` reader - 3:0\\]
This read only register contains the accumulator value of interference magnitude(MSB 4 bits) for bcnt = 0"]
pub type IntfStatsMagAcc0MsbR = crate::FieldReader;
#[doc = "Field `intf_stats_mag_acc_0_msb` writer - 3:0\\]
This read only register contains the accumulator value of interference magnitude(MSB 4 bits) for bcnt = 0"]
pub type IntfStatsMagAcc0MsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register contains the accumulator value of interference magnitude(MSB 4 bits) for bcnt = 0"]
    #[inline(always)]
    pub fn intf_stats_mag_acc_0_msb(&self) -> IntfStatsMagAcc0MsbR {
        IntfStatsMagAcc0MsbR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This read only register contains the accumulator value of interference magnitude(MSB 4 bits) for bcnt = 0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_acc_0_msb(&mut self) -> IntfStatsMagAcc0MsbW<IntfStatsMagAcc0MsbSpec> {
        IntfStatsMagAcc0MsbW::new(self, 0)
    }
}
#[doc = "INTF_STATS_MAG_ACC_0_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_0_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_0_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsMagAcc0MsbSpec;
impl crate::RegisterSpec for IntfStatsMagAcc0MsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_mag_acc_0_msb::R`](R) reader structure"]
impl crate::Readable for IntfStatsMagAcc0MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_mag_acc_0_msb::W`](W) writer structure"]
impl crate::Writable for IntfStatsMagAcc0MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_MAG_ACC_0_MSB to value 0"]
impl crate::Resettable for IntfStatsMagAcc0MsbSpec {
    const RESET_VALUE: u32 = 0;
}
