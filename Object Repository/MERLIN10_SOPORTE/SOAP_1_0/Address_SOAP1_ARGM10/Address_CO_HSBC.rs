<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_HSBC</name>
   <tag></tag>
   <elementGuidId>8e2119f6-a7c2-4cdc-a73c-d539bec888cf</elementGuidId>
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
            &lt;calle>AVENIDA NORBERTO DE LA RIESTRA 1820&lt;/calle>
            &lt;!--Optional:-->
            &lt;cp> &lt;/cp>
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
            &lt;nivel4>CABA&lt;/nivel4>
            &lt;!--Optional:-->
            &lt;nivel5> &lt;/nivel5>
            &lt;!--Optional:-->
            &lt;numero> &lt;/numero>
            &lt;!--Optional:-->
            &lt;piso> &lt;/piso>
            &lt;!--Optional:-->
            &lt;referencia> &lt;/referencia>
            &lt;!--Optional:-->
            &lt;clientAccessCode>e519c4206513a2efaad92b031fcd9e19&lt;/clientAccessCode>
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



assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>calle15&lt;/name>&lt;value>AVDA N RIESTRA&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>localidad20&lt;/name>&lt;value>CABA&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>inhibido&lt;/name>&lt;value>N&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>PORTAL&lt;/name>&lt;value>PORTAL&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitud&lt;/name>&lt;value>-34.646383&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>cpa&lt;/name>&lt;value>C1437HIU&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>concepto&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>REFERENCIA&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>inMaq&lt;/name>&lt;value>0&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>NISE&lt;/name>&lt;value>3&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.607161&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitud&lt;/name>&lt;value>-58.435269&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.445288&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>spanishCompatibility&lt;/name>&lt;value>SI&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>idCliente&lt;/name>&lt;value>180477&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>nivelRiesgo&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>tipoGeo&lt;/name>&lt;value>1&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>calle24&lt;/name>&lt;value>AVDA NORBERTO RIESTRA&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>zonaRiesgo&lt;/name>&lt;value>S&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;calle>AVDA NORBERTO DE LA RIESTRA&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;numero>1820&lt;/numero>')
assertThat(response.getResponseText()).contains('&lt;piso>&lt;/piso>')
assertThat(response.getResponseText()).contains('&lt;depto>&lt;/depto>')
assertThat(response.getResponseText()).contains('&lt;cp>1437&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;nivel2>CAPITAL FEDERAL&lt;/nivel2>')
assertThat(response.getResponseText()).contains('&lt;nivel3>CAPITAL FEDERAL&lt;/nivel3>')
assertThat(response.getResponseText()).contains('&lt;nivel4>CIUDAD AUTONOMA BUENOS AIRES&lt;/nivel4>')
assertThat(response.getResponseText()).contains('&lt;nivel5>FLORES&lt;/nivel5>')
assertThat(response.getResponseText()).contains('&lt;tramo>&lt;/tramo>')
assertThat(response.getResponseText()).contains('&lt;entreCalle1>PRES CAMILO TORRES Y TENORIO&lt;/entreCalle1>')
assertThat(response.getResponseText()).contains('&lt;entreCalle2>AGUSTIN DE VEDIA&lt;/entreCalle2>')
assertThat(response.getResponseText()).contains('&lt;referencia>&lt;/referencia>')
assertThat(response.getResponseText()).contains('&lt;observacion>&lt;/observacion>')
assertThat(response.getResponseText()).contains('&lt;locale>&lt;/locale>')
assertThat(response.getResponseText()).contains('&lt;estado>CO&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;motivo>SM&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;nivel1>&lt;/nivel1>')




</verificationScript>
   <wsdlAddress>${Address_SOAP1}</wsdlAddress>
</WebServiceRequestEntity>
