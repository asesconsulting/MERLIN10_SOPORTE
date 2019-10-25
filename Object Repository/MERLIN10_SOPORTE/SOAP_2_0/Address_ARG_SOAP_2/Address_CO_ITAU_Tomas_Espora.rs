<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_ITAU_Tomas_Espora</name>
   <tag></tag>
   <elementGuidId>441796d7-6a0b-461f-a023-64752ba688e9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize2>
         &lt;!--Optional:-->
         &lt;addressNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>a3f3627ac0b9d3310df2cbbcf661de81&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddress>
               &lt;!--Optional:-->
               &lt;level1>&lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>ADROGUE&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>AVDA CNEL DE MARINA TOMAS ESPORA 1080&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber>&lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>&lt;/postalCode>
               &lt;!--Optional:-->
               &lt;additionalData>&lt;/additionalData>
               &lt;!--Optional:-->
               &lt;betweenStreet1>&lt;/betweenStreet1>
               &lt;!--Optional:-->
               &lt;betweenStreet2>&lt;/betweenStreet2>
            &lt;/xAddress>
         &lt;/addressNormalize>
      &lt;/soap:addressNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_ARG2</defaultValue>
      <description></description>
      <id>1398d2bc-b06a-4075-891a-de3127067976</id>
      <masked>false</masked>
      <name>Address_ARG2</name>
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

/**verificacion test cases**/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'ALMIRANTE BROWN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'ADROGUE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'ALMIRANTE BROWN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA CNEL DE MARINA TOMAS ESPORA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1080')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.802560')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.388598')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1846')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'B1846BTV')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1900')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'GRAL JOSE MARIA PAZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'SOLIER')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.800384&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.393019&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedPostalCode&lt;/name>&lt;value>1846&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>7&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVENIDA CORONEL TOMAS ESPORA&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedHouseNumber&lt;/name>&lt;value>1080&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedFloor&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVENIDA CORONEL TOMAS ESPORA&lt;/value>')

/**verificacion WS unitaria


WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'ALMIRANTE BROWN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'ADROGUE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'ALMIRANTE BROWN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA CNEL DE MARINA TOMAS ESPORA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1080')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.802560')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.388598')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1846')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'B1846BTV')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1900')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.800384&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.393019&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedPostalCode&lt;/name>&lt;value>1846&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>7&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVENIDA CORONEL TOMAS ESPORA&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedHouseNumber&lt;/name>&lt;value>1080&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedFloor&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVENIDA CORONEL TOMAS ESPORA&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
