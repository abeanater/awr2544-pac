#[doc = "Register `MPFCR` reader"]
pub type R = crate::R<MpfcrSpec>;
#[doc = "Register `MPFCR` writer"]
pub type W = crate::W<MpfcrSpec>;
#[doc = "Field `MPFCLR` reader - 0:0\\]
Fault Clear register CPU write of ΓÇÿ1ΓÇÖ to the MPFCLR bit causes any error conditions stored in MPFAR and MPFSR registers to be cleared. CPU write of ΓÇÿ0ΓÇÖ has no effect.."]
pub type MpfclrR = crate::BitReader;
#[doc = "Field `MPFCLR` writer - 0:0\\]
Fault Clear register CPU write of ΓÇÿ1ΓÇÖ to the MPFCLR bit causes any error conditions stored in MPFAR and MPFSR registers to be cleared. CPU write of ΓÇÿ0ΓÇÖ has no effect.."]
pub type MpfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES1` reader - 31:1\\]
Reserved"]
pub type Res1R = crate::FieldReader<u32>;
#[doc = "Field `RES1` writer - 31:1\\]
Reserved"]
pub type Res1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Fault Clear register CPU write of ΓÇÿ1ΓÇÖ to the MPFCLR bit causes any error conditions stored in MPFAR and MPFSR registers to be cleared. CPU write of ΓÇÿ0ΓÇÖ has no effect.."]
    #[inline(always)]
    pub fn mpfclr(&self) -> MpfclrR {
        MpfclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn res1(&self) -> Res1R {
        Res1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Fault Clear register CPU write of ΓÇÿ1ΓÇÖ to the MPFCLR bit causes any error conditions stored in MPFAR and MPFSR registers to be cleared. CPU write of ΓÇÿ0ΓÇÖ has no effect.."]
    #[inline(always)]
    #[must_use]
    pub fn mpfclr(&mut self) -> MpfclrW<MpfcrSpec> {
        MpfclrW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> Res1W<MpfcrSpec> {
        Res1W::new(self, 1)
    }
}
#[doc = "Memory Protect Fault Command\n\nYou can [`read`](crate::Reg::read) this register and get [`mpfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpfcrSpec;
impl crate::RegisterSpec for MpfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpfcr::R`](R) reader structure"]
impl crate::Readable for MpfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mpfcr::W`](W) writer structure"]
impl crate::Writable for MpfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPFCR to value 0"]
impl crate::Resettable for MpfcrSpec {
    const RESET_VALUE: u32 = 0;
}
