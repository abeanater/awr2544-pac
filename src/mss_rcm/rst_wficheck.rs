#[doc = "Register `RST_WFICHECK` reader"]
pub type R = crate::R<RstWficheckSpec>;
#[doc = "Register `RST_WFICHECK` writer"]
pub type W = crate::W<RstWficheckSpec>;
#[doc = "Field `r5ssa` reader - 2:0\\]
writing '000' will disable check for WFI before global reset assertion of CR5A"]
pub type R5ssaR = crate::FieldReader;
#[doc = "Field `r5ssa` writer - 2:0\\]
writing '000' will disable check for WFI before global reset assertion of CR5A"]
pub type R5ssaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r5ssb` reader - 10:8\\]
RESERVED: Dont Use"]
pub type R5ssbR = crate::FieldReader;
#[doc = "Field `r5ssb` writer - 10:8\\]
RESERVED: Dont Use"]
pub type R5ssbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r5a` reader - 18:16\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
pub type R5aR = crate::FieldReader;
#[doc = "Field `r5a` writer - 18:16\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
pub type R5aW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r5b` reader - 26:24\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
pub type R5bR = crate::FieldReader;
#[doc = "Field `r5b` writer - 26:24\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
pub type R5bW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '000' will disable check for WFI before global reset assertion of CR5A"]
    #[inline(always)]
    pub fn r5ssa(&self) -> R5ssaR {
        R5ssaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn r5ssb(&self) -> R5ssbR {
        R5ssbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
    #[inline(always)]
    pub fn r5a(&self) -> R5aR {
        R5aR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
    #[inline(always)]
    pub fn r5b(&self) -> R5bR {
        R5bR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '000' will disable check for WFI before global reset assertion of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn r5ssa(&mut self) -> R5ssaW<RstWficheckSpec> {
        R5ssaW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn r5ssb(&mut self) -> R5ssbW<RstWficheckSpec> {
        R5ssbW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn r5a(&mut self) -> R5aW<RstWficheckSpec> {
        R5aW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
writing '000' will disable check for WFI before local reset assertion of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn r5b(&mut self) -> R5bW<RstWficheckSpec> {
        R5bW::new(self, 24)
    }
}
#[doc = "RST_WFICHECK\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_wficheck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_wficheck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstWficheckSpec;
impl crate::RegisterSpec for RstWficheckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_wficheck::R`](R) reader structure"]
impl crate::Readable for RstWficheckSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_wficheck::W`](W) writer structure"]
impl crate::Writable for RstWficheckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_WFICHECK to value 0"]
impl crate::Resettable for RstWficheckSpec {
    const RESET_VALUE: u32 = 0;
}
