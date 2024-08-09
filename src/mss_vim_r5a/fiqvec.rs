#[doc = "Register `FIQVEC` reader"]
pub type R = crate::R<FiqvecSpec>;
#[doc = "Register `FIQVEC` writer"]
pub type W = crate::W<FiqvecSpec>;
#[doc = "Field `RES22` reader - 1:0\\]
RESERVE FIELD"]
pub type Res22R = crate::FieldReader;
#[doc = "Field `RES22` writer - 1:0\\]
RESERVE FIELD"]
pub type Res22W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDR` reader - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized FIQ Register valid flag is true."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized FIQ Register valid flag is true."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res22(&self) -> Res22R {
        Res22R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized FIQ Register valid flag is true."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res22(&mut self) -> Res22W<FiqvecSpec> {
        Res22W::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Upper 30 bits of the 32-bit vector address. Only valid if the Prioritized FIQ Register valid flag is true."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<FiqvecSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "The FIQ Vector Address Register contains the 32-bit address of the interrupt vector for the current pendind FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqvec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqvec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiqvecSpec;
impl crate::RegisterSpec for FiqvecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiqvec::R`](R) reader structure"]
impl crate::Readable for FiqvecSpec {}
#[doc = "`write(|w| ..)` method takes [`fiqvec::W`](W) writer structure"]
impl crate::Writable for FiqvecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIQVEC to value 0"]
impl crate::Resettable for FiqvecSpec {
    const RESET_VALUE: u32 = 0;
}
