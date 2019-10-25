<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DC</name>
   <tag></tag>
   <elementGuidId>572c5b75-1c2d-4950-a089-de7c382d21e0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize>
         &lt;!--Optional:-->
         &lt;address>
            &lt;!--Optional:-->
            &lt;calle>barrio MASCHWITZ 1387&lt;/calle>
            &lt;!--Optional:-->
            &lt;cp>1623&lt;/cp>
            &lt;!--Optional:-->
            &lt;depto> &lt;/depto>
            &lt;!--Optional:-->
            &lt;entreCalle1> &lt;/entreCalle1>
            &lt;!--Optional:-->
            &lt;entreCalle2> &lt;/entreCalle2>
            &lt;!--Optional:-->
            &lt;nivel1> &lt;/nivel1>
            &lt;!--Optional:-->
            &lt;nivel10> &lt;/nivel10>
            &lt;!--Optional:-->
            &lt;nivel2> &lt;/nivel2>
            &lt;!--Optional:-->
            &lt;nivel3> &lt;/nivel3>
            &lt;!--Optional:-->
            &lt;nivel4>INGENIERO MASCHWITZ&lt;/nivel4>
            &lt;!--Optional:-->
            &lt;nivel5> &lt;/nivel5>
            &lt;!--Optional:-->
            &lt;numero> &lt;/numero>
            &lt;!--Optional:-->
            &lt;piso> &lt;/piso>
            &lt;!--Optional:-->
            &lt;referencia> &lt;/referencia>
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
         &lt;/address>
      &lt;/soap:addressNormalize>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_SOAP1</defaultValue>
      <description></description>
      <id>5d615922-f727-434f-9342-571d3ee00077</id>
      <masked>false</masked>
      <name>Address_SOAP1</name>
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



assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.750027&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>NISE&lt;/name>&lt;value>2&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>nivel4Abreviada15&lt;/name>&lt;value>ING MASCHWITCHZ&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.383353&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.383353&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>tipoGeo&lt;/name>&lt;value>6&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>spanishCompatibility&lt;/name>&lt;value>SI&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitud&lt;/name>&lt;value>-34.383353&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitud&lt;/name>&lt;value>-58.750027&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.750027&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;calle>BARRIO MASCHWITZ 1387&lt;/calle>&lt;numero>&lt;/numero>&lt;piso>&lt;/piso>&lt;depto>&lt;/depto>&lt;cp>1623&lt;/cp>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>ESCOBAR&lt;/nivel3>&lt;nivel4>INGENIERO MASCHWITZ&lt;/nivel4>&lt;nivel5>&lt;/nivel5>&lt;tramo>&lt;/tramo>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>0&lt;/alturaDesde>&lt;alturaHasta>0&lt;/alturaHasta>&lt;calle>BO MASCHWITZ PRIVADO&lt;/calle>&lt;cp>1619&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>ESCOBAR&lt;/nivel3>&lt;nivel4>MAQUINISTA F SAVIO&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>0&lt;/alturaDesde>&lt;alturaHasta>0&lt;/alturaHasta>&lt;calle>BO MASCHWITZ VILLAGE&lt;/calle>&lt;cp>1619&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>ESCOBAR&lt;/nivel3>&lt;nivel4>INGENIERO MASCHWITZ&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>0&lt;/alturaDesde>&lt;alturaHasta>0&lt;/alturaHasta>&lt;calle>BO MASCHWITZ CLUB&lt;/calle>&lt;cp>1619&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>ESCOBAR&lt;/nivel3>&lt;nivel4>INGENIERO MASCHWITZ&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;referencia>&lt;/referencia>')
assertThat(response.getResponseText()).contains('&lt;observacion>1387&lt;/observacion>')
assertThat(response.getResponseText()).contains('&lt;locale>&lt;/locale>')
assertThat(response.getResponseText()).contains('&lt;cantidadResoluciones>3&lt;/cantidadResoluciones>')
assertThat(response.getResponseText()).contains('&lt;estado>DU&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;motivo>DC&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;nivel1>&lt;/nivel1>')




</verificationScript>
   <wsdlAddress>${Address_SOAP1}</wsdlAddress>
</WebServiceRequestEntity>
