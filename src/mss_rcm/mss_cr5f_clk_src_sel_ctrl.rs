#[doc = "Register `MSS_CR5F_CLK_SRC_SEL_CTRL` reader"]
pub type R = crate::R<MssCr5fClkSrcSelCtrlSpec>;
#[doc = "Register `MSS_CR5F_CLK_SRC_SEL_CTRL` writer"]
pub type W = crate::W<MssCr5fClkSrcSelCtrlSpec>;
#[doc = "Field `clksrcsel` reader - 2:0\\]
writing 3'b111 ensures R5 to be same as BUS_CLK writing 3'b000 ensures R5 clock will be same as CR5_CLK from top_rcm"]
pub type ClksrcselR = crate::FieldReader;
#[doc = "Field `clksrcsel` writer - 2:0\\]
writing 3'b111 ensures R5 to be same as BUS_CLK writing 3'b000 ensures R5 clock will be same as CR5_CLK from top_rcm"]
pub type ClksrcselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b111 ensures R5 to be same as BUS_CLK writing 3'b000 ensures R5 clock will be same as CR5_CLK from top_rcm"]
    #[inline(always)]
    pub fn clksrcsel(&self) -> ClksrcselR {
        ClksrcselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b111 ensures R5 to be same as BUS_CLK writing 3'b000 ensures R5 clock will be same as CR5_CLK from top_rcm"]
    #[inline(always)]
    #[must_use]
    pub fn clksrcsel(&mut self) -> ClksrcselW<MssCr5fClkSrcSelCtrlSpec> {
        ClksrcselW::new(self, 0)
    }
}
#[doc = "MSS_CR5F_CLK_SRC_SEL_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5f_clk_src_sel_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5f_clk_src_sel_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssCr5fClkSrcSelCtrlSpec;
impl crate::RegisterSpec for MssCr5fClkSrcSelCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_cr5f_clk_src_sel_ctrl::R`](R) reader structure"]
impl crate::Readable for MssCr5fClkSrcSelCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_cr5f_clk_src_sel_ctrl::W`](W) writer structure"]
impl crate::Writable for MssCr5fClkSrcSelCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_CR5F_CLK_SRC_SEL_CTRL to value 0"]
impl crate::Resettable for MssCr5fClkSrcSelCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
