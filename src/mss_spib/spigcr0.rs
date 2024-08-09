#[doc = "Register `SPIGCR0` reader"]
pub type R = crate::R<Spigcr0Spec>;
#[doc = "Register `SPIGCR0` writer"]
pub type W = crate::W<Spigcr0Spec>;
#[doc = "Field `nRESET` reader - 0:0\\]
This is the local reset control for the module. This bit needs to be set to ΓÇÿ1ΓÇÖ before any operation on SPI / MibSPI can be done. Only after setting this bit to ΓÇÿ1ΓÇÖ, the Auto Initialization of Multibuffer RAM starts. Clearing this bit to ΓÇÿ0ΓÇÖ will result in all of the control and status register values to return to their default values. 0 = SPI / MibSPI is in reset state 1 = SPI / MibSPI is out of reset state."]
pub type NResetR = crate::BitReader;
#[doc = "Field `nRESET` writer - 0:0\\]
This is the local reset control for the module. This bit needs to be set to ΓÇÿ1ΓÇÖ before any operation on SPI / MibSPI can be done. Only after setting this bit to ΓÇÿ1ΓÇÖ, the Auto Initialization of Multibuffer RAM starts. Clearing this bit to ΓÇÿ0ΓÇÖ will result in all of the control and status register values to return to their default values. 0 = SPI / MibSPI is in reset state 1 = SPI / MibSPI is out of reset state."]
pub type NResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:1\\]
Reserved, Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:1\\]
Reserved, Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This is the local reset control for the module. This bit needs to be set to ΓÇÿ1ΓÇÖ before any operation on SPI / MibSPI can be done. Only after setting this bit to ΓÇÿ1ΓÇÖ, the Auto Initialization of Multibuffer RAM starts. Clearing this bit to ΓÇÿ0ΓÇÖ will result in all of the control and status register values to return to their default values. 0 = SPI / MibSPI is in reset state 1 = SPI / MibSPI is out of reset state."]
    #[inline(always)]
    pub fn n_reset(&self) -> NResetR {
        NResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This is the local reset control for the module. This bit needs to be set to ΓÇÿ1ΓÇÖ before any operation on SPI / MibSPI can be done. Only after setting this bit to ΓÇÿ1ΓÇÖ, the Auto Initialization of Multibuffer RAM starts. Clearing this bit to ΓÇÿ0ΓÇÖ will result in all of the control and status register values to return to their default values. 0 = SPI / MibSPI is in reset state 1 = SPI / MibSPI is out of reset state."]
    #[inline(always)]
    #[must_use]
    pub fn n_reset(&mut self) -> NResetW<Spigcr0Spec> {
        NResetW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spigcr0Spec> {
        NuW::new(self, 1)
    }
}
#[doc = "SPI / MibSPI Global Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spigcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spigcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spigcr0Spec;
impl crate::RegisterSpec for Spigcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spigcr0::R`](R) reader structure"]
impl crate::Readable for Spigcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`spigcr0::W`](W) writer structure"]
impl crate::Writable for Spigcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIGCR0 to value 0"]
impl crate::Resettable for Spigcr0Spec {
    const RESET_VALUE: u32 = 0;
}
