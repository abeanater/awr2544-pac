#[doc = "Register `R5_AHB_EN` reader"]
pub type R = crate::R<R5AhbEnSpec>;
#[doc = "Register `R5_AHB_EN` writer"]
pub type W = crate::W<R5AhbEnSpec>;
#[doc = "Field `cpu0_ahb_init` reader - 2:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
pub type Cpu0AhbInitR = crate::FieldReader;
#[doc = "Field `cpu0_ahb_init` writer - 2:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
pub type Cpu0AhbInitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpu1_ahb_init` reader - 18:16\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
pub type Cpu1AhbInitR = crate::FieldReader;
#[doc = "Field `cpu1_ahb_init` writer - 18:16\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
pub type Cpu1AhbInitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
    #[inline(always)]
    pub fn cpu0_ahb_init(&self) -> Cpu0AhbInitR {
        Cpu0AhbInitR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
    #[inline(always)]
    pub fn cpu1_ahb_init(&self) -> Cpu1AhbInitR {
        Cpu1AhbInitR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_ahb_init(&mut self) -> Cpu0AhbInitW<R5AhbEnSpec> {
        Cpu0AhbInitW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_ahb_init(&mut self) -> Cpu1AhbInitW<R5AhbEnSpec> {
        Cpu1AhbInitW::new(self, 16)
    }
}
#[doc = "R5_AHB_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_ahb_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_ahb_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5AhbEnSpec;
impl crate::RegisterSpec for R5AhbEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_ahb_en::R`](R) reader structure"]
impl crate::Readable for R5AhbEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_ahb_en::W`](W) writer structure"]
impl crate::Writable for R5AhbEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_AHB_EN to value 0"]
impl crate::Resettable for R5AhbEnSpec {
    const RESET_VALUE: u32 = 0;
}
