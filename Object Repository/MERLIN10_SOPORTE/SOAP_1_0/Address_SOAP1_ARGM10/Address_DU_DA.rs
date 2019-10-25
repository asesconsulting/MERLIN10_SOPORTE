<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DA</name>
   <tag></tag>
   <elementGuidId>00c432dd-8673-4083-9baa-8e2a3143706c</elementGuidId>
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
            &lt;calle>&quot;SAN JOSE DE FLORES 6001&lt;/calle>
            &lt;!--Optional:-->
            &lt;cp>1653&lt;/cp>
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
            &lt;nivel4>Villa Ballester&lt;/nivel4>
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



assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.557521&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>PORTAL&lt;/name>&lt;value>PORTAL&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>Relevance&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>idCliente&lt;/name>&lt;value>797&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>MatchLevel&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.542114&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>REFERENCIA&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>latitud&lt;/name>&lt;value>00.000000&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>spanishCompatibility&lt;/name>&lt;value>SI&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>cpa&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>tipoGeo&lt;/name>&lt;value>&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>longitud&lt;/name>&lt;value>00.000000&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;calle>SAN JOSE DE FLORES&lt;/calle>&lt;numero>6001&lt;/numero>&lt;piso>&lt;/piso>&lt;depto>&lt;/depto>&lt;cp>1653&lt;/cp>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>&lt;/nivel3>&lt;nivel4>VILLA BALLESTER&lt;/nivel4>&lt;nivel5>&lt;/nivel5>&lt;tramo>&lt;/tramo>&lt;entreCalle1>&lt;/entreCalle1>&lt;entreCalle2>&lt;/entreCalle2>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>6201&lt;/alturaDesde>&lt;alturaHasta>6900&lt;/alturaHasta>&lt;calle>SAN JOSE DE FLORES&lt;/calle>&lt;cp>&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>&lt;/nivel3>&lt;nivel4>VILLA GODOY CRUZ&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>4701&lt;/alturaDesde>&lt;alturaHasta>6200&lt;/alturaHasta>&lt;calle>SAN JOSE DE FLORES&lt;/calle>&lt;cp>&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>&lt;/nivel3>&lt;nivel4>VILLA GENERAL JOSE TOMAS GUIDO&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>1301&lt;/alturaDesde>&lt;alturaHasta>1600&lt;/alturaHasta>&lt;calle>SAN JOSE&lt;/calle>&lt;cp>&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>&lt;/nivel3>&lt;nivel4>VILLA GRANADEROS DE SAN MARTIN&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>1201&lt;/alturaDesde>&lt;alturaHasta>1800&lt;/alturaHasta>&lt;calle>SAN JOSE&lt;/calle>&lt;cp>&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>&lt;/nivel3>&lt;nivel4>VILLA GENERAL EUGENIO NECOCHEA&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>2001&lt;/alturaDesde>&lt;alturaHasta>2300&lt;/alturaHasta>&lt;calle>SAN JOSE&lt;/calle>&lt;cp>&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>&lt;/nivel3>&lt;nivel4>VILLA BALLESTER&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;doubtfulAddress>&lt;alturaDesde>5801&lt;/alturaDesde>&lt;alturaHasta>6199&lt;/alturaHasta>&lt;calle>SAN JOSE DE FLORES&lt;/calle>&lt;cp>1653&lt;/cp>&lt;nivel1>&lt;/nivel1>&lt;nivel2>BUENOS AIRES&lt;/nivel2>&lt;nivel3>GENERAL SAN MARTIN&lt;/nivel3>&lt;nivel4>VILLA GENERAL ANTONIO JOSE DE SUCRE&lt;/nivel4>&lt;observaciones>&lt;/observaciones>&lt;/doubtfulAddress>')
assertThat(response.getResponseText()).contains('&lt;referencia>&lt;/referencia>')
assertThat(response.getResponseText()).contains('&lt;observacion>&lt;/observacion>')
assertThat(response.getResponseText()).contains('&lt;locale>&lt;/locale>')
assertThat(response.getResponseText()).contains('&lt;cantidadResoluciones>6&lt;/cantidadResoluciones>')
assertThat(response.getResponseText()).contains('&lt;estado>DU&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;motivo>DA&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;nivel1>&lt;/nivel1>')




</verificationScript>
   <wsdlAddress>${Address_SOAP1}</wsdlAddress>
</WebServiceRequestEntity>
