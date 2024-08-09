#[doc = "Register `EFUSE_OVERRIDE_IP1_BG1_MAG` reader"]
pub type R = crate::R<EfuseOverrideIp1Bg1MagSpec>;
#[doc = "Register `EFUSE_OVERRIDE_IP1_BG1_MAG` writer"]
pub type W = crate::W<EfuseOverrideIp1Bg1MagSpec>;
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
    pub fn override_(&mut self) -> OverrideW<EfuseOverrideIp1Bg1MagSpec> {
        OverrideW::new(self, 0)
    }
}
#[doc = "EFUSE_OVERRIDE_IP1_BG1_MAG\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_ip1_bg1_mag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_ip1_bg1_mag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseOverrideIp1Bg1MagSpec;
impl crate::RegisterSpec for EfuseOverrideIp1Bg1MagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_override_ip1_bg1_mag::R`](R) reader structure"]
impl crate::Readable for EfuseOverrideIp1Bg1MagSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse_override_ip1_bg1_mag::W`](W) writer structure"]
impl crate::Writable for EfuseOverrideIp1Bg1MagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_OVERRIDE_IP1_BG1_MAG to value 0"]
impl crate::Resettable for EfuseOverrideIp1Bg1MagSpec {
    const RESET_VALUE: u32 = 0;
}
