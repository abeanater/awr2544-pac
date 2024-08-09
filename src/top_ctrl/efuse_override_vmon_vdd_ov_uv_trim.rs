#[doc = "Register `EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM` reader"]
pub type R = crate::R<EfuseOverrideVmonVddOvUvTrimSpec>;
#[doc = "Register `EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM` writer"]
pub type W = crate::W<EfuseOverrideVmonVddOvUvTrimSpec>;
#[doc = "Field `override` reader - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value. Refer to the ANAREG in TOP_RCM for the override value"]
pub type OverrideR = crate::FieldReader;
#[doc = "Field `override` writer - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value. Refer to the ANAREG in TOP_RCM for the override value"]
pub type OverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value. Refer to the ANAREG in TOP_RCM for the override value"]
    #[inline(always)]
    pub fn override_(&self) -> OverrideR {
        OverrideR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value. Refer to the ANAREG in TOP_RCM for the override value"]
    #[inline(always)]
    #[must_use]
    pub fn override_(&mut self) -> OverrideW<EfuseOverrideVmonVddOvUvTrimSpec> {
        OverrideW::new(self, 0)
    }
}
#[doc = "EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_vmon_vdd_ov_uv_trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_vmon_vdd_ov_uv_trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseOverrideVmonVddOvUvTrimSpec;
impl crate::RegisterSpec for EfuseOverrideVmonVddOvUvTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_override_vmon_vdd_ov_uv_trim::R`](R) reader structure"]
impl crate::Readable for EfuseOverrideVmonVddOvUvTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse_override_vmon_vdd_ov_uv_trim::W`](W) writer structure"]
impl crate::Writable for EfuseOverrideVmonVddOvUvTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM to value 0"]
impl crate::Resettable for EfuseOverrideVmonVddOvUvTrimSpec {
    const RESET_VALUE: u32 = 0;
}
