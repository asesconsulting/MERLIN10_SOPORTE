<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_Ruta</name>
   <tag></tag>
   <elementGuidId>b1c46de6-55f2-4479-bc29-3061ecd3c45b</elementGuidId>
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
            &lt;calle>ruta provincial 53 km 13.5&lt;/calle>
            &lt;!--Optional:-->
            &lt;cp> &lt;/cp>
            &lt;!--Optional:-->
            &lt;depto> &lt;/depto>
            &lt;!--Optional:-->
            &lt;entreCalle1> &lt;/entreCalle1>
            &lt;!--Optional:-->
            &lt;entreCalle2> &lt;/entreCalle2>
            &lt;!--Optional:-->
            &lt;nivel1>&lt;/nivel1>
            &lt;!--Optional:-->
            &lt;nivel10> &lt;/nivel10>
            &lt;!--Optional:-->
            &lt;nivel2>buenos aires&lt;/nivel2>
            &lt;!--Optional:-->
            &lt;nivel3> &lt;/nivel3>
            &lt;!--Optional:-->
            &lt;nivel4>Villa San Luis&lt;/nivel4>
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
               &lt;customAdapter>&lt;/customAdapter>
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



assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.255838&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>inhibido&lt;/name>&lt;value>N&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>PORTAL&lt;/name>&lt;value>PORTAL&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>Relevance&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>idCliente&lt;/name>&lt;value>797&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitud&lt;/name>&lt;value>-58.275859&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>MatchLevel&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>concepto&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>concepto&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.860420&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>inMaq&lt;/name>&lt;value>0&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>NISE&lt;/name>&lt;value>2&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>spanishCompatibility&lt;/name>&lt;value>SI&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>nivelRiesgo&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>cpa&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>puerta&lt;/name>&lt;value>SI&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>tipoGeo&lt;/name>&lt;value>1&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitud&lt;/name>&lt;value>-34.889679&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;calle>RUTA PROV 53&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;numero>13,5&lt;/numero>')
assertThat(response.getResponseText()).contains('&lt;piso>&lt;/piso>')
assertThat(response.getResponseText()).contains('&lt;depto>&lt;/depto>')
assertThat(response.getResponseText()).contains('&lt;cp>1888&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;nivel2>BUENOS AIRES&lt;/nivel2>')
assertThat(response.getResponseText()).contains('&lt;nivel3>FLORENCIO VARELA&lt;/nivel3>')
assertThat(response.getResponseText()).contains('&lt;nivel4>VILLA SAN LUIS&lt;/nivel4>')
assertThat(response.getResponseText()).contains('&lt;nivel5>VILLA SAN LUIS&lt;/nivel5>')
assertThat(response.getResponseText()).contains('&lt;tramo>&lt;/tramo>')
assertThat(response.getResponseText()).contains('&lt;entreCalle1>&lt;/entreCalle1>')
assertThat(response.getResponseText()).contains('&lt;entreCalle2>&lt;/entreCalle2>')
assertThat(response.getResponseText()).contains('&lt;referencia>&lt;/referencia>')
assertThat(response.getResponseText()).contains('&lt;observacion>&lt;/observacion>')
assertThat(response.getResponseText()).contains('&lt;locale>&lt;/locale>')
assertThat(response.getResponseText()).contains('&lt;estado>CO&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;motivo>SM&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;nivel1>&lt;/nivel1>')




</verificationScript>
   <wsdlAddress>${Address_SOAP1}</wsdlAddress>
</WebServiceRequestEntity>
