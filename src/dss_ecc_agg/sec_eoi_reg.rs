#[doc = "Register `sec_eoi_reg` reader"]
pub type R = crate::R<SecEoiRegSpec>;
#[doc = "Register `sec_eoi_reg` writer"]
pub type W = crate::W<SecEoiRegSpec>;
#[doc = "Field `EOI_REGISTER` reader - 0:0\\]
EOI Register"]
pub type EoiRegisterR = crate::BitReader;
#[doc = "Field `EOI_REGISTER` writer - 0:0\\]
EOI Register"]
pub type EoiRegisterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EOI Register"]
    #[inline(always)]
    pub fn eoi_register(&self) -> EoiRegisterR {
        EoiRegisterR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EOI Register"]
    #[inline(always)]
    #[must_use]
    pub fn eoi_register(&mut self) -> EoiRegisterW<SecEoiRegSpec> {
        EoiRegisterW::new(self, 0)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_eoi_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_eoi_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEoiRegSpec;
impl crate::RegisterSpec for SecEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_eoi_reg::R`](R) reader structure"]
impl crate::Readable for SecEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_eoi_reg::W`](W) writer structure"]
impl crate::Writable for SecEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sec_eoi_reg to value 0"]
impl crate::Resettable for SecEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
