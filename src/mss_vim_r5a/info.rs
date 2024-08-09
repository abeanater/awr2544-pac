#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Register `INFO` writer"]
pub type W = crate::W<InfoSpec>;
#[doc = "Field `INTERRUPTS` reader - 10:0\\]
Total number of Interrupts"]
pub type InterruptsR = crate::FieldReader<u16>;
#[doc = "Field `INTERRUPTS` writer - 10:0\\]
Total number of Interrupts"]
pub type InterruptsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RES1` reader - 31:11\\]
RESERVE FIELD"]
pub type Res1R = crate::FieldReader<u32>;
#[doc = "Field `RES1` writer - 31:11\\]
RESERVE FIELD"]
pub type Res1W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Total number of Interrupts"]
    #[inline(always)]
    pub fn interrupts(&self) -> InterruptsR {
        InterruptsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res1(&self) -> Res1R {
        Res1R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Total number of Interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupts(&mut self) -> InterruptsW<InfoSpec> {
        InterruptsW::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> Res1W<InfoSpec> {
        Res1W::new(self, 11)
    }
}
#[doc = "The Info Register gives the configuration Inforrmation of this VIM.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`write(|w| ..)` method takes [`info::W`](W) writer structure"]
impl crate::Writable for InfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for InfoSpec {
    const RESET_VALUE: u32 = 0;
}
