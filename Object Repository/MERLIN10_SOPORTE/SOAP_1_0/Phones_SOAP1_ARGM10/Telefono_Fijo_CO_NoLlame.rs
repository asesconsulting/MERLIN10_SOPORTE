<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Telefono_Fijo_CO_NoLlame</name>
   <tag></tag>
   <elementGuidId>a91eb36c-9442-44ef-b106-fa394c3673a8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.phoneonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:phoneNormalize>
         &lt;!--Optional:-->
         &lt;phone>
            &lt;!--Optional:-->
            &lt;caracteristica> &lt;/caracteristica>
            &lt;!--Optional:-->
            &lt;completeNumber>01144420390&lt;/completeNumber>
            &lt;!--Optional:-->
            &lt;cp> &lt;/cp>
            &lt;!--Optional:-->
            &lt;ddi> &lt;/ddi>
            &lt;!--Optional:-->
            &lt;ddn> &lt;/ddn>
            &lt;!--Optional:-->
            &lt;nivel1> &lt;/nivel1>
            &lt;!--Optional:-->
            &lt;nivel2> &lt;/nivel2>
            &lt;!--Optional:-->
            &lt;nivel3> &lt;/nivel3>
            &lt;!--Optional:-->
            &lt;nivel4> &lt;/nivel4>
            &lt;!--Optional:-->
            &lt;nivel5> &lt;/nivel5>
            &lt;!--Optional:-->
            &lt;numero> &lt;/numero>
            &lt;!--Optional:-->
            &lt;observaciones> &lt;/observaciones>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;sector> &lt;/sector>
            &lt;!--Optional:-->
            &lt;userName> &lt;/userName>
         &lt;/phone>
      &lt;/soap:phoneNormalize>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>phoneNormalize</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Telefonos</defaultValue>
      <description></description>
      <id>3786ec45-84b9-4393-baae-70130e68750a</id>
      <masked>false</masked>
      <name>Telefonos</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()





assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>TEL_COMPLETO&lt;/name>&lt;value>01144420390&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>figuraEnGuia&lt;/name>&lt;value>SI&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>registroNoLlame&lt;/name>&lt;value>SI&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>CEL_COMPLETO&lt;/name>&lt;value>01144420390&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>PORTAL&lt;/name>&lt;value>PORTAL&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>telefonoCompleto&lt;/name>&lt;value>01144420390&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>validado&lt;/name>&lt;value>ok&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;nivel2>&lt;/nivel2>')
assertThat(response.getResponseText()).contains('&lt;nivel4>&lt;/nivel4>')
assertThat(response.getResponseText()).contains('&lt;cp>0&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;ddi>&lt;/ddi>')
assertThat(response.getResponseText()).contains('&lt;ddn>011&lt;/ddn>')
assertThat(response.getResponseText()).contains('&lt;caracteristica>4442&lt;/caracteristica>')
assertThat(response.getResponseText()).contains('&lt;numero>0390&lt;/numero>')
assertThat(response.getResponseText()).contains('&lt;observaciones>&lt;/observaciones>')
assertThat(response.getResponseText()).contains('&lt;estado>CO&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;motivo>&lt;/motivo>')</verificationScript>
   <wsdlAddress>${Telefonos}</wsdlAddress>
</WebServiceRequestEntity>
