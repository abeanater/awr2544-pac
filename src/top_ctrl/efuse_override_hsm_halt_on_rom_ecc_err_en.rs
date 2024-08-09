#[doc = "Register `EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN` reader"]
pub type R = crate::R<EfuseOverrideHsmHaltOnRomEccErrEnSpec>;
#[doc = "Register `EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN` writer"]
pub type W = crate::W<EfuseOverrideHsmHaltOnRomEccErrEnSpec>;
#[doc = "Field `override` reader - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type OverrideR = crate::FieldReader;
#[doc = "Field `override` writer - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type OverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `override_val` reader - 4:4\\]
Override MMR value"]
pub type OverrideValR = crate::BitReader;
#[doc = "Field `override_val` writer - 4:4\\]
Override MMR value"]
pub type OverrideValW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    pub fn override_(&self) -> OverrideR {
        OverrideR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Override MMR value"]
    #[inline(always)]
    pub fn override_val(&self) -> OverrideValR {
        OverrideValR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    #[must_use]
    pub fn override_(&mut self) -> OverrideW<EfuseOverrideHsmHaltOnRomEccErrEnSpec> {
        OverrideW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Override MMR value"]
    #[inline(always)]
    #[must_use]
    pub fn override_val(&mut self) -> OverrideValW<EfuseOverrideHsmHaltOnRomEccErrEnSpec> {
        OverrideValW::new(self, 4)
    }
}
#[doc = "EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_hsm_halt_on_rom_ecc_err_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_hsm_halt_on_rom_ecc_err_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseOverrideHsmHaltOnRomEccErrEnSpec;
impl crate::RegisterSpec for EfuseOverrideHsmHaltOnRomEccErrEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_override_hsm_halt_on_rom_ecc_err_en::R`](R) reader structure"]
impl crate::Readable for EfuseOverrideHsmHaltOnRomEccErrEnSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse_override_hsm_halt_on_rom_ecc_err_en::W`](W) writer structure"]
impl crate::Writable for EfuseOverrideHsmHaltOnRomEccErrEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN to value 0"]
impl crate::Resettable for EfuseOverrideHsmHaltOnRomEccErrEnSpec {
    const RESET_VALUE: u32 = 0;
}
