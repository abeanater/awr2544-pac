#[doc = "Register `MDIO_CLAUS45_REG` reader"]
pub type R = crate::R<MdioClaus45RegSpec>;
#[doc = "Register `MDIO_CLAUS45_REG` writer"]
pub type W = crate::W<MdioClaus45RegSpec>;
#[doc = "Field `MDIO_CLAUSE_45` reader - 31:0\\]
MDIO Clause 45"]
pub type MdioClause45R = crate::FieldReader<u32>;
#[doc = "Field `MDIO_CLAUSE_45` writer - 31:0\\]
MDIO Clause 45"]
pub type MdioClause45W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO Clause 45"]
    #[inline(always)]
    pub fn mdio_clause_45(&self) -> MdioClause45R {
        MdioClause45R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO Clause 45"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_clause_45(&mut self) -> MdioClause45W<MdioClaus45RegSpec> {
        MdioClause45W::new(self, 0)
    }
}
#[doc = "MDIO Clause45 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_claus45_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_claus45_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioClaus45RegSpec;
impl crate::RegisterSpec for MdioClaus45RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_claus45_reg::R`](R) reader structure"]
impl crate::Readable for MdioClaus45RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_claus45_reg::W`](W) writer structure"]
impl crate::Writable for MdioClaus45RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_CLAUS45_REG to value 0"]
impl crate::Resettable for MdioClaus45RegSpec {
    const RESET_VALUE: u32 = 0;
}
