#[doc = "Register `EFUSE_OVERRIDE_LVDS_BGAP_TRIM` reader"]
pub type R = crate::R<EfuseOverrideLvdsBgapTrimSpec>;
#[doc = "Register `EFUSE_OVERRIDE_LVDS_BGAP_TRIM` writer"]
pub type W = crate::W<EfuseOverrideLvdsBgapTrimSpec>;
#[doc = "Field `override` reader - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type OverrideR = crate::FieldReader;
#[doc = "Field `override` writer - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type OverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `override_val` reader - 9:4\\]
Override MMR value"]
pub type OverrideValR = crate::FieldReader;
#[doc = "Field `override_val` writer - 9:4\\]
Override MMR value"]
pub type OverrideValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    pub fn override_(&self) -> OverrideR {
        OverrideR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Override MMR value"]
    #[inline(always)]
    pub fn override_val(&self) -> OverrideValR {
        OverrideValR::new(((self.bits >> 4) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    #[must_use]
    pub fn override_(&mut self) -> OverrideW<EfuseOverrideLvdsBgapTrimSpec> {
        OverrideW::new(self, 0)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Override MMR value"]
    #[inline(always)]
    #[must_use]
    pub fn override_val(&mut self) -> OverrideValW<EfuseOverrideLvdsBgapTrimSpec> {
        OverrideValW::new(self, 4)
    }
}
#[doc = "EFUSE_OVERRIDE_LVDS_BGAP_TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_lvds_bgap_trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_lvds_bgap_trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseOverrideLvdsBgapTrimSpec;
impl crate::RegisterSpec for EfuseOverrideLvdsBgapTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_override_lvds_bgap_trim::R`](R) reader structure"]
impl crate::Readable for EfuseOverrideLvdsBgapTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse_override_lvds_bgap_trim::W`](W) writer structure"]
impl crate::Writable for EfuseOverrideLvdsBgapTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_OVERRIDE_LVDS_BGAP_TRIM to value 0"]
impl crate::Resettable for EfuseOverrideLvdsBgapTrimSpec {
    const RESET_VALUE: u32 = 0;
}
