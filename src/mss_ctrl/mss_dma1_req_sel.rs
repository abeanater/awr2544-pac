#[doc = "Register `MSS_DMA1_REQ_SEL` reader"]
pub type R = crate::R<MssDma1ReqSelSpec>;
#[doc = "Register `MSS_DMA1_REQ_SEL` writer"]
pub type W = crate::W<MssDma1ReqSelSpec>;
#[doc = "Field `select` reader - 31:0\\]
Reserved for R&amp;D. Do not touch"]
pub type SelectR = crate::FieldReader<u32>;
#[doc = "Field `select` writer - 31:0\\]
Reserved for R&amp;D. Do not touch"]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for R&amp;D. Do not touch"]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for R&amp;D. Do not touch"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SelectW<MssDma1ReqSelSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "MSS_DMA1_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dma1_req_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dma1_req_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDma1ReqSelSpec;
impl crate::RegisterSpec for MssDma1ReqSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dma1_req_sel::R`](R) reader structure"]
impl crate::Readable for MssDma1ReqSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dma1_req_sel::W`](W) writer structure"]
impl crate::Writable for MssDma1ReqSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMA1_REQ_SEL to value 0"]
impl crate::Resettable for MssDma1ReqSelSpec {
    const RESET_VALUE: u32 = 0;
}
