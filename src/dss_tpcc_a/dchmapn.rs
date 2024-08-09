#[doc = "Register `DCHMAPN` reader"]
pub type R = crate::R<DchmapnSpec>;
#[doc = "Register `DCHMAPN` writer"]
pub type W = crate::W<DchmapnSpec>;
#[doc = "Field `PAENTRY` reader - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
pub type PaentryR = crate::FieldReader<u16>;
#[doc = "Field `PAENTRY` writer - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
pub type PaentryW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 5:13 - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
    #[inline(always)]
    pub fn paentry(&self) -> PaentryR {
        PaentryR::new((self.bits >> 5) & 0x01ff)
    }
}
impl W {
    #[doc = "Bits 5:13 - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
    #[inline(always)]
    #[must_use]
    pub fn paentry(&mut self) -> PaentryW<DchmapnSpec> {
        PaentryW::new(self, 5)
    }
}
#[doc = "DMA Channel N Mapping to Param Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dchmapn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dchmapn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DchmapnSpec;
impl crate::RegisterSpec for DchmapnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dchmapn::R`](R) reader structure"]
impl crate::Readable for DchmapnSpec {}
#[doc = "`write(|w| ..)` method takes [`dchmapn::W`](W) writer structure"]
impl crate::Writable for DchmapnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DCHMAPN to value 0"]
impl crate::Resettable for DchmapnSpec {
    const RESET_VALUE: u16 = 0;
}
