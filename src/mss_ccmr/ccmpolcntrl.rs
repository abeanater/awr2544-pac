#[doc = "Register `CCMPOLCNTRL` reader"]
pub type R = crate::R<CcmpolcntrlSpec>;
#[doc = "Register `CCMPOLCNTRL` writer"]
pub type W = crate::W<CcmpolcntrlSpec>;
#[doc = "Field `POL_INV` reader - 7:0\\]
This value is used to invert the 8 XOR of the CPU1 to create compare fail in functional active compare mode. User and privilege mode read = Returns current value of the POL INV Privilege mode write = Update the values of POL INV"]
pub type PolInvR = crate::FieldReader;
#[doc = "Field `POL_INV` writer - 7:0\\]
This value is used to invert the 8 XOR of the CPU1 to create compare fail in functional active compare mode. User and privilege mode read = Returns current value of the POL INV Privilege mode write = Update the values of POL INV"]
pub type PolInvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU12` reader - 31:8\\]
Reserved"]
pub type Nu12R = crate::FieldReader<u32>;
#[doc = "Field `NU12` writer - 31:8\\]
Reserved"]
pub type Nu12W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This value is used to invert the 8 XOR of the CPU1 to create compare fail in functional active compare mode. User and privilege mode read = Returns current value of the POL INV Privilege mode write = Update the values of POL INV"]
    #[inline(always)]
    pub fn pol_inv(&self) -> PolInvR {
        PolInvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu12(&self) -> Nu12R {
        Nu12R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This value is used to invert the 8 XOR of the CPU1 to create compare fail in functional active compare mode. User and privilege mode read = Returns current value of the POL INV Privilege mode write = Update the values of POL INV"]
    #[inline(always)]
    #[must_use]
    pub fn pol_inv(&mut self) -> PolInvW<CcmpolcntrlSpec> {
        PolInvW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu12(&mut self) -> Nu12W<CcmpolcntrlSpec> {
        Nu12W::new(self, 8)
    }
}
#[doc = "CPU Compare Polarity Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmpolcntrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmpolcntrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcmpolcntrlSpec;
impl crate::RegisterSpec for CcmpolcntrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmpolcntrl::R`](R) reader structure"]
impl crate::Readable for CcmpolcntrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmpolcntrl::W`](W) writer structure"]
impl crate::Writable for CcmpolcntrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMPOLCNTRL to value 0"]
impl crate::Resettable for CcmpolcntrlSpec {
    const RESET_VALUE: u32 = 0;
}
