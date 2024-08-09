#[doc = "Register `SYSRST_BY_DBG_RST` reader"]
pub type R = crate::R<SysrstByDbgRstSpec>;
#[doc = "Register `SYSRST_BY_DBG_RST` writer"]
pub type W = crate::W<SysrstByDbgRstSpec>;
#[doc = "Field `r5a` reader - 2:0\\]
writing '111' will block debug reset request from CR5A toggling globally reset for CR5A"]
pub type R5aR = crate::FieldReader;
#[doc = "Field `r5a` writer - 2:0\\]
writing '111' will block debug reset request from CR5A toggling globally reset for CR5A"]
pub type R5aW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r5b` reader - 18:16\\]
RESERVED: Dont Use"]
pub type R5bR = crate::FieldReader;
#[doc = "Field `r5b` writer - 18:16\\]
RESERVED: Dont Use"]
pub type R5bW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will block debug reset request from CR5A toggling globally reset for CR5A"]
    #[inline(always)]
    pub fn r5a(&self) -> R5aR {
        R5aR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn r5b(&self) -> R5bR {
        R5bR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will block debug reset request from CR5A toggling globally reset for CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn r5a(&mut self) -> R5aW<SysrstByDbgRstSpec> {
        R5aW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn r5b(&mut self) -> R5bW<SysrstByDbgRstSpec> {
        R5bW::new(self, 16)
    }
}
#[doc = "SYSRST_BY_DBG_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrst_by_dbg_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrst_by_dbg_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysrstByDbgRstSpec;
impl crate::RegisterSpec for SysrstByDbgRstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysrst_by_dbg_rst::R`](R) reader structure"]
impl crate::Readable for SysrstByDbgRstSpec {}
#[doc = "`write(|w| ..)` method takes [`sysrst_by_dbg_rst::W`](W) writer structure"]
impl crate::Writable for SysrstByDbgRstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSRST_BY_DBG_RST to value 0"]
impl crate::Resettable for SysrstByDbgRstSpec {
    const RESET_VALUE: u32 = 0;
}
