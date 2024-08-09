#[doc = "Register `DMACNTLEN` reader"]
pub type R = crate::R<DmacntlenSpec>;
#[doc = "Register `DMACNTLEN` writer"]
pub type W = crate::W<DmacntlenSpec>;
#[doc = "Field `LARGE_COUNT` reader - 0:0\\]
0: Writes to the DMAxCTRL register will modify the ICOUNT value. Reading ICOUNT and COUNT can be done from the DMAxCTRL register. The DMAxCOUNT register should not be used since any write to this register will be overwritten by a subsequent write to DMAxCTRL register to set the TXDMAENA or RXDMAENA bits. 1: Writes to the DMAxCTRL register will not modify the ICOUNT value. The ICOUNT value must be written to in the DMAxCOUNT register before the RXDMAENA or TXDMAENA bits are set in the DMAxCTRL register. The DMAxCOUNT register should be used for reading COUNT or ICOUNT."]
pub type LargeCountR = crate::BitReader;
#[doc = "Field `LARGE_COUNT` writer - 0:0\\]
0: Writes to the DMAxCTRL register will modify the ICOUNT value. Reading ICOUNT and COUNT can be done from the DMAxCTRL register. The DMAxCOUNT register should not be used since any write to this register will be overwritten by a subsequent write to DMAxCTRL register to set the TXDMAENA or RXDMAENA bits. 1: Writes to the DMAxCTRL register will not modify the ICOUNT value. The ICOUNT value must be written to in the DMAxCOUNT register before the RXDMAENA or TXDMAENA bits are set in the DMAxCTRL register. The DMAxCOUNT register should be used for reading COUNT or ICOUNT."]
pub type LargeCountW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:1\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:1\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Writes to the DMAxCTRL register will modify the ICOUNT value. Reading ICOUNT and COUNT can be done from the DMAxCTRL register. The DMAxCOUNT register should not be used since any write to this register will be overwritten by a subsequent write to DMAxCTRL register to set the TXDMAENA or RXDMAENA bits. 1: Writes to the DMAxCTRL register will not modify the ICOUNT value. The ICOUNT value must be written to in the DMAxCOUNT register before the RXDMAENA or TXDMAENA bits are set in the DMAxCTRL register. The DMAxCOUNT register should be used for reading COUNT or ICOUNT."]
    #[inline(always)]
    pub fn large_count(&self) -> LargeCountR {
        LargeCountR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Writes to the DMAxCTRL register will modify the ICOUNT value. Reading ICOUNT and COUNT can be done from the DMAxCTRL register. The DMAxCOUNT register should not be used since any write to this register will be overwritten by a subsequent write to DMAxCTRL register to set the TXDMAENA or RXDMAENA bits. 1: Writes to the DMAxCTRL register will not modify the ICOUNT value. The ICOUNT value must be written to in the DMAxCOUNT register before the RXDMAENA or TXDMAENA bits are set in the DMAxCTRL register. The DMAxCOUNT register should be used for reading COUNT or ICOUNT."]
    #[inline(always)]
    #[must_use]
    pub fn large_count(&mut self) -> LargeCountW<DmacntlenSpec> {
        LargeCountW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<DmacntlenSpec> {
        NuW::new(self, 1)
    }
}
#[doc = "DMA LARGE COUNT register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacntlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacntlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacntlenSpec;
impl crate::RegisterSpec for DmacntlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacntlen::R`](R) reader structure"]
impl crate::Readable for DmacntlenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacntlen::W`](W) writer structure"]
impl crate::Writable for DmacntlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACNTLEN to value 0"]
impl crate::Resettable for DmacntlenSpec {
    const RESET_VALUE: u32 = 0;
}
