#[doc = "Register `CH23_HIL_CP_OVERRIDE` reader"]
pub type R = crate::R<Ch23HilCpOverrideSpec>;
#[doc = "Register `CH23_HIL_CP_OVERRIDE` writer"]
pub type W = crate::W<Ch23HilCpOverrideSpec>;
#[doc = "Field `chirp2` reader - 15:0\\]
Override data used for Chirp2. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp2R = crate::FieldReader<u16>;
#[doc = "Field `chirp2` writer - 15:0\\]
Override data used for Chirp2. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `chirp3` reader - 31:16\\]
Override data used for Chirp3. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp3R = crate::FieldReader<u16>;
#[doc = "Field `chirp3` writer - 31:16\\]
Override data used for Chirp3. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp2. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp2(&self) -> Chirp2R {
        Chirp2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp3. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp3(&self) -> Chirp3R {
        Chirp3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp2. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp2(&mut self) -> Chirp2W<Ch23HilCpOverrideSpec> {
        Chirp2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp3. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp3(&mut self) -> Chirp3W<Ch23HilCpOverrideSpec> {
        Chirp3W::new(self, 16)
    }
}
#[doc = "CH23_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_hil_cp_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_hil_cp_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch23HilCpOverrideSpec;
impl crate::RegisterSpec for Ch23HilCpOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch23_hil_cp_override::R`](R) reader structure"]
impl crate::Readable for Ch23HilCpOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`ch23_hil_cp_override::W`](W) writer structure"]
impl crate::Writable for Ch23HilCpOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH23_HIL_CP_OVERRIDE to value 0"]
impl crate::Resettable for Ch23HilCpOverrideSpec {
    const RESET_VALUE: u32 = 0;
}
