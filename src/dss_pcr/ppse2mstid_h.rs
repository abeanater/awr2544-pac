#[doc = "Register `PPSE2MSTID_H` reader"]
pub type R = crate::R<Ppse2mstidHSpec>;
#[doc = "Register `PPSE2MSTID_H` writer"]
pub type W = crate::W<Ppse2mstidHSpec>;
#[doc = "Field `PPSE2_QUAD2_MSTID` reader - 15:0\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppse2Quad2MstidR = crate::FieldReader<u16>;
#[doc = "Field `PPSE2_QUAD2_MSTID` writer - 15:0\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppse2Quad2MstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PPSE2_QUAD3_MSTID` reader - 31:16\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppse2Quad3MstidR = crate::FieldReader<u16>;
#[doc = "Field `PPSE2_QUAD3_MSTID` writer - 31:16\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppse2Quad3MstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn ppse2_quad2_mstid(&self) -> Ppse2Quad2MstidR {
        Ppse2Quad2MstidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn ppse2_quad3_mstid(&self) -> Ppse2Quad3MstidR {
        Ppse2Quad3MstidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn ppse2_quad2_mstid(&mut self) -> Ppse2Quad2MstidW<Ppse2mstidHSpec> {
        Ppse2Quad2MstidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each quadrant in PPSE frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn ppse2_quad3_mstid(&mut self) -> Ppse2Quad3MstidW<Ppse2mstidHSpec> {
        Ppse2Quad3MstidW::new(self, 16)
    }
}
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register2_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse2mstid_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse2mstid_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppse2mstidHSpec;
impl crate::RegisterSpec for Ppse2mstidHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppse2mstid_h::R`](R) reader structure"]
impl crate::Readable for Ppse2mstidHSpec {}
#[doc = "`write(|w| ..)` method takes [`ppse2mstid_h::W`](W) writer structure"]
impl crate::Writable for Ppse2mstidHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPSE2MSTID_H to value 0"]
impl crate::Resettable for Ppse2mstidHSpec {
    const RESET_VALUE: u32 = 0;
}
