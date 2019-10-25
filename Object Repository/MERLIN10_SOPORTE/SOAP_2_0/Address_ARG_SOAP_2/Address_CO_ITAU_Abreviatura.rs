<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_ITAU_Abreviatura</name>
   <tag></tag>
   <elementGuidId>d75a13d6-2f7f-4611-acc5-1185add8cfa2</elementGuidId>
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
               &lt;level4>caba&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>AVDA GRAL INDALECIO CHENAUT&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber>1833&lt;/houseNumber>
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

/**verificacion test case**/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'CIUDAD AUTONOMA BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'PALERMO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA GRAL INDALECIO CHENAUT')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1833')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.570875')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.431900')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1426')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'C1426DIB')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1701')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '2000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'MANUEL JOSE BAEZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'ESTEBAN ARCE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedUnit&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedHouseNumber&lt;/name>&lt;value>1833&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedPostalCode&lt;/name>&lt;value>1426&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.445288&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVDA GRAL INDALECIO CHENAUT&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedFloor&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>9&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.607161&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedPostalCode&lt;/name>&lt;value>1426&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedHouseNumber&lt;/name>&lt;value>1833&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet20&lt;/name>&lt;value>AV GRAL I CHENAUT&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedFloor&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedUnit&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVDA GRAL INDALECIO CHENAUT&lt;/value>')

/**verificacion WS unitaria

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'CIUDAD AUTONOMA BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'PALERMO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA GRAL INDALECIO CHENAUT')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1833')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.570875')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.431900')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1426')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'C1426DIB')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1701')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '2000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'MANUEL JOSE BAEZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'ESTEBAN ARCE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedUnit&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedHouseNumber&lt;/name>&lt;value>1833&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedPostalCode&lt;/name>&lt;value>1426&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.445288&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVDA GRAL INDALECIO CHENAUT&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedFloor&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>9&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.607161&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedPostalCode&lt;/name>&lt;value>1426&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedHouseNumber&lt;/name>&lt;value>1833&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet20&lt;/name>&lt;value>AV GRAL I CHENAUT&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedFloor&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedUnit&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet&lt;/name>&lt;value>AVDA GRAL INDALECIO CHENAUT&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
