#[doc = "Register `MSS_MII10_CLK_STATUS` reader"]
pub type R = crate::R<MssMii10ClkStatusSpec>;
#[doc = "Register `MSS_MII10_CLK_STATUS` writer"]
pub type W = crate::W<MssMii10ClkStatusSpec>;
#[doc = "Field `currdivider` reader - 15:8\\]
Status shows the current divider value choosen for MII10"]
pub type CurrdividerR = crate::FieldReader;
#[doc = "Field `currdivider` writer - 15:8\\]
Status shows the current divider value choosen for MII10"]
pub type CurrdividerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
Status shows the current divider value choosen for MII10"]
    #[inline(always)]
    pub fn currdivider(&self) -> CurrdividerR {
        CurrdividerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Status shows the current divider value choosen for MII10"]
    #[inline(always)]
    #[must_use]
    pub fn currdivider(&mut self) -> CurrdividerW<MssMii10ClkStatusSpec> {
        CurrdividerW::new(self, 8)
    }
}
#[doc = "MSS_MII10_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mii10_clk_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mii10_clk_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssMii10ClkStatusSpec;
impl crate::RegisterSpec for MssMii10ClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_mii10_clk_status::R`](R) reader structure"]
impl crate::Readable for MssMii10ClkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_mii10_clk_status::W`](W) writer structure"]
impl crate::Writable for MssMii10ClkStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_MII10_CLK_STATUS to value 0"]
impl crate::Resettable for MssMii10ClkStatusSpec {
    const RESET_VALUE: u32 = 0;
}
