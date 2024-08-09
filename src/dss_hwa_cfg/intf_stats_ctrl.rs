#[doc = "Register `INTF_STATS_CTRL` reader"]
pub type R = crate::R<IntfStatsCtrlSpec>;
#[doc = "Register `INTF_STATS_CTRL` writer"]
pub type W = crate::W<IntfStatsCtrlSpec>;
#[doc = "Field `intf_stats_mag_shift` reader - 2:0\\]
Right shift applied after scaling - 2^(6+INTERSUM_MAGS_SHIFT). Can t be more than 2^(12)."]
pub type IntfStatsMagShiftR = crate::FieldReader;
#[doc = "Field `intf_stats_mag_shift` writer - 2:0\\]
Right shift applied after scaling - 2^(6+INTERSUM_MAGS_SHIFT). Can t be more than 2^(12)."]
pub type IntfStatsMagShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `intf_stats_magdiff_shift` reader - 6:4\\]
Right shift applied after scaling - 2^(6+INTERFSUM_MAGDIFF_SHIFT). Can t be more than 2^(12)."]
pub type IntfStatsMagdiffShiftR = crate::FieldReader;
#[doc = "Field `intf_stats_magdiff_shift` writer - 6:4\\]
Right shift applied after scaling - 2^(6+INTERFSUM_MAGDIFF_SHIFT). Can t be more than 2^(12)."]
pub type IntfStatsMagdiffShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `intf_stats_mag_scale` reader - 23:16\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block. Default 8= scale of 1.0"]
pub type IntfStatsMagScaleR = crate::FieldReader;
#[doc = "Field `intf_stats_mag_scale` writer - 23:16\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block. Default 8= scale of 1.0"]
pub type IntfStatsMagScaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `intf_stats_magdiff_scale` reader - 31:24\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block. Default 8= scale of 1.0"]
pub type IntfStatsMagdiffScaleR = crate::FieldReader;
#[doc = "Field `intf_stats_magdiff_scale` writer - 31:24\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block. Default 8= scale of 1.0"]
pub type IntfStatsMagdiffScaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Right shift applied after scaling - 2^(6+INTERSUM_MAGS_SHIFT). Can t be more than 2^(12)."]
    #[inline(always)]
    pub fn intf_stats_mag_shift(&self) -> IntfStatsMagShiftR {
        IntfStatsMagShiftR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Right shift applied after scaling - 2^(6+INTERFSUM_MAGDIFF_SHIFT). Can t be more than 2^(12)."]
    #[inline(always)]
    pub fn intf_stats_magdiff_shift(&self) -> IntfStatsMagdiffShiftR {
        IntfStatsMagdiffShiftR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block. Default 8= scale of 1.0"]
    #[inline(always)]
    pub fn intf_stats_mag_scale(&self) -> IntfStatsMagScaleR {
        IntfStatsMagScaleR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block. Default 8= scale of 1.0"]
    #[inline(always)]
    pub fn intf_stats_magdiff_scale(&self) -> IntfStatsMagdiffScaleR {
        IntfStatsMagdiffScaleR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Right shift applied after scaling - 2^(6+INTERSUM_MAGS_SHIFT). Can t be more than 2^(12)."]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_shift(&mut self) -> IntfStatsMagShiftW<IntfStatsCtrlSpec> {
        IntfStatsMagShiftW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Right shift applied after scaling - 2^(6+INTERFSUM_MAGDIFF_SHIFT). Can t be more than 2^(12)."]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_magdiff_shift(&mut self) -> IntfStatsMagdiffShiftW<IntfStatsCtrlSpec> {
        IntfStatsMagdiffShiftW::new(self, 4)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block. Default 8= scale of 1.0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_scale(&mut self) -> IntfStatsMagScaleW<IntfStatsCtrlSpec> {
        IntfStatsMagScaleW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block. Default 8= scale of 1.0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_magdiff_scale(&mut self) -> IntfStatsMagdiffScaleW<IntfStatsCtrlSpec> {
        IntfStatsMagdiffScaleW::new(self, 24)
    }
}
#[doc = "INTF_STATS_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsCtrlSpec;
impl crate::RegisterSpec for IntfStatsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_ctrl::R`](R) reader structure"]
impl crate::Readable for IntfStatsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_ctrl::W`](W) writer structure"]
impl crate::Writable for IntfStatsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_CTRL to value 0"]
impl crate::Resettable for IntfStatsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
