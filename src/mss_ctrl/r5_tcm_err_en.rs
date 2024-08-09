#[doc = "Register `R5_TCM_ERR_EN` reader"]
pub type R = crate::R<R5TcmErrEnSpec>;
#[doc = "Register `R5_TCM_ERR_EN` writer"]
pub type W = crate::W<R5TcmErrEnSpec>;
#[doc = "Field `cpu0_tcm` reader - 2:0\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
pub type Cpu0TcmR = crate::FieldReader;
#[doc = "Field `cpu0_tcm` writer - 2:0\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
pub type Cpu0TcmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpu1_tcm` reader - 18:16\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
pub type Cpu1TcmR = crate::FieldReader;
#[doc = "Field `cpu1_tcm` writer - 18:16\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
pub type Cpu1TcmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
    #[inline(always)]
    pub fn cpu0_tcm(&self) -> Cpu0TcmR {
        Cpu0TcmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
    #[inline(always)]
    pub fn cpu1_tcm(&self) -> Cpu1TcmR {
        Cpu1TcmR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_tcm(&mut self) -> Cpu0TcmW<R5TcmErrEnSpec> {
        Cpu0TcmW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Ti internal Register. Modifying this register is not recommended TCMs ECC check enable. Tie each bit high to enable ECC checking on appropraite TCM"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_tcm(&mut self) -> Cpu1TcmW<R5TcmErrEnSpec> {
        Cpu1TcmW::new(self, 16)
    }
}
#[doc = "R5_TCM_ERR_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_tcm_err_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_tcm_err_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5TcmErrEnSpec;
impl crate::RegisterSpec for R5TcmErrEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_tcm_err_en::R`](R) reader structure"]
impl crate::Readable for R5TcmErrEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_tcm_err_en::W`](W) writer structure"]
impl crate::Writable for R5TcmErrEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_TCM_ERR_EN to value 0"]
impl crate::Resettable for R5TcmErrEnSpec {
    const RESET_VALUE: u32 = 0;
}
